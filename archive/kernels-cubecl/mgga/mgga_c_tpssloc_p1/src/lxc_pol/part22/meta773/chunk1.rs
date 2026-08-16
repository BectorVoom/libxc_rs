//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2645/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2645<F: Float>(t53777: F, t53779: F, t56099: F, t56102: F, t56104: F, t20396: F, t67: F, t758: F, t53798: F, t5397: F, t606: F, t584: F) -> (F, F, F, F, F, F, F, F, F) {
    let t73958 = F::cast_from(0.65061487801810439052e-1_f64) * t53777;
    let t73959 = F::cast_from(0.97592231702715658578e-1_f64) * t53779;
    let t73960 = F::cast_from(0.51947577317044391276e2_f64) * t56099;
    let t73961 = F::cast_from(0.17544670867903938621e1_f64) * t56102;
    let t73962 = F::cast_from(0.17544670867903938621e1_f64) * t56104;
    let t73967 = t20396 * t67 * t758;
    let t73968 = F::cast_from(0.18311447306006545054e-3_f64) * t73967;
    let t73969 = F::cast_from(0.10526802520742363173e2_f64) * t53798;
    let t73975 = t5397 * t606;
    let t73978 = t584 * t5397;
    (t73958, t73959, t73960, t73961, t73962, t73968, t73969, t73975, t73978)
}
