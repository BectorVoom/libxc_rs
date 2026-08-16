//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1014/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1014(t24462: f64, t6534: f64, t131: f64, t2108: f64, t39063: f64, t8662: f64, t31867: f64, t9239: f64, t2240: f64, t24503: f64, t8301: f64, t39049: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t116008 = 27.0_f64 * t24462 * t6534;
    let t116065 = t2108 * t131;
    let t116075 = t39063 * t8662;
    let t116082 = t9239 * t31867;
    let t116088 = t2240 * t8301 * t24503;
    let t116096 = t39049 * t8662;
    (t116008, t116065, t116075, t116082, t116088, t116096)
}
