//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 775/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk775<F: Float>(t1009: F, t6544: F, t2836: F, t300: F, t3247: F, t6302: F, t6432: F, t6484: F, t6489: F, t6494: F, t6499: F, t6502: F, t6506: F, t6510: F, t979: F, t3187: F, t3188: F, t6276: F, t8: F) -> (F, F, F) {
    let t6545 = t6544 * t1009;
    let t6548 = 0.66725e-1 * t979 * t6302 + 0.24872916666666666666e-2 * t6484 + 0.49745833333333333332e-2 * t6489 - 0.33163888888888888888e-2 * t6494 - 0.55273148148148148147e-3 * t6499 + 0.33163888888888888888e-2 * t6502 + 0.16581944444444444444e-2 * t6506 + 0.27636574074074074073e-2 * t6510 - t3247 + t6432 * t300 + 0.890445125e-2 * t2836 * t6302 - 0.66725e-1 * t979 * t6545;
    let t6555 = t6276 * t8 - t3187 - t3188;
    (t6545, t6548, t6555)
}
