//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 586/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk586<F: Float>(t1001: F, t6539: F, t286: F, t285: F, t2879: F, t4937: F, t4959: F, t6518: F, t6522: F, t6526: F, t6530: F, t6535: F, t991: F, t1009: F, t2836: F, t300: F, t3247: F, t6302: F, t6432: F, t6484: F, t6489: F, t6494: F, t6499: F, t6502: F, t6506: F, t6510: F, t979: F) -> (F, F, F, F, F) {
    let t6540 = t1001 * t6539;
    let t6541 = t286 * t6540;
    let t6544 = -t2879 + t4937 / 432.0 - t4959 / 144.0 + t991 * t6518 / 216.0 - t991 * t6522 / 144.0 - t991 * t6526 / 144.0 + t991 * t6530 / 288.0 + t285 * t6535 / 48.0 - t285 * t6541 / 96.0;
    let t6545 = t6544 * t1009;
    let t6548 = 0.66725e-1 * t979 * t6302 + 0.24872916666666666666e-2 * t6484 + 0.49745833333333333332e-2 * t6489 - 0.33163888888888888888e-2 * t6494 - 0.55273148148148148147e-3 * t6499 + 0.33163888888888888888e-2 * t6502 + 0.16581944444444444444e-2 * t6506 + 0.27636574074074074073e-2 * t6510 - t3247 + t6432 * t300 + 0.890445125e-2 * t2836 * t6302 - 0.66725e-1 * t979 * t6545;
    (t6540, t6541, t6544, t6545, t6548)
}
