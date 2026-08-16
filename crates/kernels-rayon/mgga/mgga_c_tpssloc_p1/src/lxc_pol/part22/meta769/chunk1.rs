//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2611/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2611(t1213: f64, t22244: f64, t248: f64, t3570: f64, t1227: f64, t21758: f64, t45268: f64, t11692: f64, t11697: f64, t22283: f64, t1216: f64, t15498: f64, t15569: f64, t1653: f64, t18360: f64, t18584: f64, t18941: f64, t19056: f64, t22301: f64, t22309: f64, t22314: f64, t3515: f64, t3577: f64, t3578: f64, t44858: f64, t44896: f64, t44965: f64, t45119: f64, t4582: f64, t5012: f64, t52897: f64, t53000: f64, t6203: f64, t72767: f64) -> f64 {
    let t72849 = t1213 * t248 * t3570 * t22244;
    let t72857 = t1227 * t248 * t45268 * t21758;
    let t72864 = t11692 * t11697 * t22283;
    let t72878 = t53000 - 5.0_f64 / 864.0_f64 * t15498 * t6203 + t44896 * t22309 / 512.0_f64 + t72849 / 4608.0_f64 - t44858 * t22314 / 512.0_f64 + t44965 * t22301 / 3072.0_f64 - 5.0_f64 / 7776.0_f64 * t72857 - t3577 * t3578 * t18941 * t1653 / 1536.0_f64 + t72864 / 2304.0_f64 + t15569 * t18584 / 144.0_f64 + t15569 * t18360 / 144.0_f64 + t45119 * t52897 * t72767 * t1216 / 1024.0_f64 - t3515 * t4582 * t19056 * t5012 / 1024.0_f64;
    t72878
}
