//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2063/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2063<F: Float>(t82122: F, t214: F, t2710: F, t23258: F, t6547: F, t794: F, t852: F, t6562: F, t6572: F, t23219: F, t23265: F, t23030: F, t23208: F) -> (F, F, F, F, F, F, F, F) {
    let t82123 = F::cast_from(0.16220877603642232915e0_f64) * t82122;
    let t82124 = t214 * t2710;
    let t82131 = t6547 * t23258;
    let t82133 = t794 * t852;
    let t82135 = t6562 * t82133 * t6572;
    let t82143 = t6547 * t23219;
    let t82145 = t6547 * t23265;
    let t82147 = t23030 * t23208;
    (t82123, t82124, t82131, t82133, t82135, t82143, t82145, t82147)
}
