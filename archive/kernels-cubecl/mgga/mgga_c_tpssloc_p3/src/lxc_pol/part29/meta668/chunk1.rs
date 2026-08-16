//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2231/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2231<F: Float>(t1369: F, t91160: F, t26257: F, t3876: F, t1831: F, t80849: F, t7712: F, t80939: F, t22683: F, t26285: F, t6546: F, t16148: F, t221: F, t26284: F) -> (F, F, F, F, F, F) {
    let t91161 = t91160 * t1369;
    let t91162 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t91161;
    let t91163 = t26257 * t3876;
    let t91165 = t80849 * t1831;
    let t91167 = t80939 * t7712;
    let t91170 = t6546 * t22683 * t26285;
    let t91171 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t91170;
    let t91173 = t26284 * t221 * t16148;
    (t91162, t91163, t91165, t91167, t91171, t91173)
}
