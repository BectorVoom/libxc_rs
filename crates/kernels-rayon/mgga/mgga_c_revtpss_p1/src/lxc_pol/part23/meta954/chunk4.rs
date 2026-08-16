//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3177/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3177(t12772: f64, t24786: f64, t3625: f64, t1248: f64, t13046: f64, t13053: f64, t17396: f64, t21166: f64, t24619: f64, t24834: f64, t3626: f64, t3720: f64, t44500: f64, t44578: f64, t44952: f64, t45371: f64, t45386: f64, t471: f64, t5407: f64, t56947: f64, t56953: f64, t57422: f64, t6429: f64, t6690: f64, t70794: f64, t70995: f64, t82838: f64) -> f64 {
    let t83435 = t3625 * t12772 * t24786;
    let t83451 = 0.68598428988911579154e-2_f64 * t56953 * t6690 + 0.68598428988911579154e-2_f64 * t17396 * t21166 - 0.12862205435420921092e-2_f64 * t45386 * t24619 - t57422 - 0.42874018118069736972e-3_f64 * t3625 * t3626 * t6429 * t82838 - 0.64311027177104605458e-3_f64 * t45371 * t3720 * t24834 * t70794 * t471 - 0.14481890564325777821e-1_f64 * t70995 * t5407 - 0.28582678745379824648e-3_f64 * t83435 - 0.38586616306262763276e-2_f64 * t44500 * t3720 * t24834 * t13046 * t1248 + 0.38586616306262763276e-2_f64 * t44578 * t3720 * t24834 * t13053 * t1248 - 0.12862205435420921092e-2_f64 * t44952 * t3720 * t24834 * t56947;
    t83451
}
