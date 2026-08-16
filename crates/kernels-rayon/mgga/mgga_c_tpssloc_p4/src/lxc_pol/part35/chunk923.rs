//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 923/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk923(t1171: f64, t6109: f64, t6011: f64, t699: f64, t6014: f64, t6017: f64, t135: f64, t6146: f64, t1174: f64, t6140: f64, t4889: f64, t4916: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18489 = t6109 * t1171;
    let t18494 = t699 * t6011;
    let t18505 = t699 * t6014;
    let t18512 = t699 * t6017;
    let t18529 = t135 * t6146;
    let t18530 = t1174 * t18529;
    let t18532 = t135 * t6140;
    let t18533 = t1174 * t18532;
    let t18536 = t4889 * t4916;
    (t18489, t18494, t18505, t18512, t18530, t18533, t18536)
}
