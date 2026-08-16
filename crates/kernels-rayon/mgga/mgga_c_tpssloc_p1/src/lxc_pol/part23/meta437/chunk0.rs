//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1279/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1279(t11570: f64, t20234: f64, t18457: f64, t4889: f64, t18321: f64, t4896: f64, t18451: f64, t1174: f64, t22081: f64, t44562: f64, t22046: f64, t3431: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73225 = t11570 * t20234;
    let t73272 = t4889 * t18457;
    let t73274 = t18321 * t4896;
    let t73276 = t4889 * t18451;
    let t73279 = t1174 * t44562 * t22081;
    let t73287 = t1174 * t3431 * t22046;
    (t73225, t73272, t73274, t73276, t73279, t73287)
}
