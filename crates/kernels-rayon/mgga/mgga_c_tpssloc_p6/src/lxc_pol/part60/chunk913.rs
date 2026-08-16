//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 913/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk913(t229: f64, t268: f64, t6559: f64, t225: f64, t23228: f64, t2056: f64, t40772: f64, t25: f64, t1408: f64, t2752: f64, t1519: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81651 = t6559 * t229 * t268;
    let t82074 = t23228 * t225;
    let t84766 = t2056 * t40772;
    let t86716 = t40772 * t25;
    let t86721 = t2752 * t1408;
    let t86873 = t213 * t1519 * t225;
    (t81651, t82074, t84766, t86716, t86721, t86873)
}
