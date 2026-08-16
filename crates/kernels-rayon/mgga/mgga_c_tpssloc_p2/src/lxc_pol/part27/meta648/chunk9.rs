//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2247/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2247(t25751: f64, t82431: f64, t4657: f64, t6703: f64, t7554: f64, t82573: f64, t1920: f64, t2966: f64, t7561: f64, t225: f64, t25789: f64, t1066: f64, t13742: f64, t1635: f64, t1956: f64, t23346: f64, t23394: f64, t23588: f64, t25407: f64, t25732: f64, t3169: f64, t4542: f64, t50653: f64, t50690: f64, t6687: f64, t6704: f64, t6706: f64, t82402: f64, t83398: f64, t83408: f64) -> f64 {
    let t89597 = 0.18277045187202515961e-2_f64 * t82431 * t25751;
    let t89598 = t6703 * t4657;
    let t89609 = t82573 * t7554;
    let t89617 = t1920 * t2966 * t7561;
    let t89620 = t25789 * t225;
    let t89623 = -2.0_f64 * t50653 * t1956 - 0.27415567780803773942e-2_f64 * t83398 + 0.14621636149762012769e-1_f64 * t82402 * t25751 - t89597 - 0.16449340668482264365e-1_f64 * t6687 * t89598 * t6706 - 2.0_f64 * t3169 * t25732 + 0.3289868133696452873e-1_f64 * t6687 * t6704 * t23394 * t13742 - t83408 * t1635 - 0.48738787165873375897e-2_f64 * t89609 + 0.43864908449286038306e-1_f64 * t23346 * t25407 + 0.16449340668482264365e-1_f64 * t6687 * t4542 * t23588 - 0.18277045187202515961e-2_f64 * t89617 - t50690 * t1956 - 2.0_f64 * t89620 * t1066;
    t89623
}
