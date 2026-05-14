//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1081/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1081<F: Float>(t19104: F, t12929: F, t12931: F, t12933: F, t12948: F, t14831: F, t19100: F, t19102: F, t19106: F, t19111: F, t19116: F, t19121: F, t19125: F, t19129: F, t19134: F, t19138: F, t19142: F) -> (F,) {
    let t21720 = 0.2283111111111111111e-1 * t19104;
    let t21730 = -t14831 - 0.1522074074074074074e-1 * t12929 + 0.38051851851851851851e-2 * t12933 - 0.11415555555555555555e-1 * t12948 + 0.57077777777777777777e-2 * t12931 - 0.76103703703703703702e-2 * t19100 + 0.76103703703703703701e-2 * t19102 - t21720 + 0.1255711111111111111e0 * t19106 - 0.19025925925925925925e-1 * t19111 + 0.68493333333333333331e-1 * t19116 - 0.45662222222222222221e-1 * t19121 - 0.11415555555555555555e-1 * t19125 - 0.10274e0 * t19129 + 0.13698666666666666666e0 * t19134 + 0.34246666666666666666e-1 * t19138 - 0.34246666666666666666e-1 * t19142;
    (t21730,)
}
