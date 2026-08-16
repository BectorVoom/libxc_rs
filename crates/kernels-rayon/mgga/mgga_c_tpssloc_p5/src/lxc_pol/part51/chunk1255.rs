//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1255/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1255(t10143: f64, t1081: f64, t28: f64, t40772: f64, t1649: f64, t2752: f64, t111: f64, t26097: f64, t1834: f64, t794: f64, t213: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t89849 = t10143 * t1081;
    let t89953 = t40772 * t28;
    let t89992 = t2752 * t1649;
    let t90400 = t26097 * t111;
    let t90544 = t794 * t1834;
    let t90566 = t213 * t1834 * t225;
    (t89849, t89953, t89992, t90400, t90544, t90566)
}
