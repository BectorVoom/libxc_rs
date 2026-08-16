//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 380/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk380(t1173: f64, t1174: f64, t1706: f64, t1710: f64, t1717: f64, t463: f64, t491: f64, t1196: f64, t1409: f64, t974: f64, t225: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1720 = -0.22222222222222222222e-2_f64 * t1706 * t463 + t1173 - 0.27777777777777777777e-3_f64 * t1174 * t1710 - 0.83333333333333333332e-3_f64 * t1174 * t1717;
    let t1721 = t1720 * t491;
    let t1725 = t1196 * t1409;
    let t1726 = t974 * t1725;
    let t1729 = t1720 * t225;
    let t1730 = t1729 * t68;
    (t1720, t1721, t1725, t1726, t1729, t1730)
}
