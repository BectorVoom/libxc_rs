//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 827/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk827(t2053: f64, t2718: f64, t4300: f64, t13463: f64, t1528: f64, t2054: f64, t23207: f64, t23209: f64, t23233: f64, t23236: f64, t24291: f64, t24305: f64, t25194: f64, t2713: f64, t4147: f64, t4268: f64, t4301: f64, t7087: f64, t7092: f64, t7107: f64, t7842: f64, t855: f64) -> (f64, f64) {
    let t26690 = t2718 * t2053 * t4300;
    let t26698 = 2.0_f64 * t4147 * t7092 + t23207 + 0.82246703342411321825e-2_f64 * t23209 - t2713 * t7842 + 2.0_f64 * t855 * t26690 - t13463 * t2054 - t7087 * t4301 - t24291 + t23233 + 0.3289868133696452873e-1_f64 * t25194 + t23236 - t24305 * t1528 - t4268 * t7107;
    (t26690, t26698)
}
