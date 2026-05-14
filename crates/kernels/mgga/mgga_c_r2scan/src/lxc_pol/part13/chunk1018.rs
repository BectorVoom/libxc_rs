//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1018/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1018<F: Float>(t39613: F, t39614: F, t7340: F, t1054: F, t6132: F, t7345: F, t6139: F, t37702: F, t37707: F, t37714: F, t39599: F, t39602: F, t39604: F, t39608: F, t39610: F, t10872: F, t11686: F) -> (F, F) {
    let t39616 = t39613 * t39614 * t7340;
    let t39619 = t6132 * t1054 * t7345;
    let t39622 = t6139 * t1054 * t7340;
    let t39624 = -0.97574405393827830186e-2 * t37702 - 0.45022119329691164872e0 * t37707 - 0.47609969197673950972e-2 * t37714 + 0.21831846657716620896e-2 * t39599 + t39602 - 0.43663693315433241792e-2 * t39604 - t39608 - 0.86682217400542685632e-1 * t39610 - 0.21951497276451705328e0 * t39616 - 0.17336443480108537126e0 * t39619 - 0.5200933044032561138e0 * t39622;
    let t39627 = t10872 * t11686;
    (t39624, t39627)
}
