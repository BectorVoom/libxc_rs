//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1009/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1009<F: Float>(t33940: F, t30157: F, t30162: F, t30185: F, t30187: F, t30201: F, t30203: F, t30212: F, t30217: F, t30220: F, t32377: F, t32379: F, t32380: F, t32384: F, t32385: F, t32386: F, t32387: F, t33947: F) -> (F,) {
    let t36870 = 0.21437009059034868486e-2 * t33940;
    let t36872 = -0.62896184579208304138e-2 * t30157 + 0.25158473831683321654e-2 * t30162 - t32377 - t32379 + t32380 + 0.37737710747524982482e-2 * t30185 - 0.80031500487063509014e-2 * t30187 + t32384 - t32385 - t32386 - t32387 - 0.94344276868812456205e-2 * t30201 + 0.64025200389650807212e-1 * t30203 - 0.25158473831683321656e-2 * t30212 - 0.5590771962596293701e-2 * t30217 + 0.42874018118069736972e-2 * t30220 - t36870 - 0.12862205435420921092e-1 * t33947;
    (t36872,)
}
