//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3177/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3177<F: Float>(t12772: F, t24786: F, t3625: F, t1248: F, t13046: F, t13053: F, t17396: F, t21166: F, t24619: F, t24834: F, t3626: F, t3720: F, t44500: F, t44578: F, t44952: F, t45371: F, t45386: F, t471: F, t5407: F, t56947: F, t56953: F, t57422: F, t6429: F, t6690: F, t70794: F, t70995: F, t82838: F) -> F {
    let t83435 = t3625 * t12772 * t24786;
    let t83451 = F::cast_from(0.68598428988911579154e-2_f64) * t56953 * t6690 + F::cast_from(0.68598428988911579154e-2_f64) * t17396 * t21166 - F::cast_from(0.12862205435420921092e-2_f64) * t45386 * t24619 - t57422 - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t3626 * t6429 * t82838 - F::cast_from(0.64311027177104605458e-3_f64) * t45371 * t3720 * t24834 * t70794 * t471 - F::cast_from(0.14481890564325777821e-1_f64) * t70995 * t5407 - F::cast_from(0.28582678745379824648e-3_f64) * t83435 - F::cast_from(0.38586616306262763276e-2_f64) * t44500 * t3720 * t24834 * t13046 * t1248 + F::cast_from(0.38586616306262763276e-2_f64) * t44578 * t3720 * t24834 * t13053 * t1248 - F::cast_from(0.12862205435420921092e-2_f64) * t44952 * t3720 * t24834 * t56947;
    t83451
}
