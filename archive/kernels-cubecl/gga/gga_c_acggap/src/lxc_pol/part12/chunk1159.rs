//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1159/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1159<F: Float>(t33940: F, t30157: F, t30162: F, t30185: F, t30187: F, t30201: F, t30203: F, t30212: F, t30217: F, t30220: F, t32377: F, t32379: F, t32380: F, t32384: F, t32385: F, t32386: F, t32387: F, t33947: F) -> F {
    let t36870 = F::cast_from(0.21437009059034868486e-2_f64) * t33940;
    let t36872 = -F::cast_from(0.62896184579208304138e-2_f64) * t30157 + F::cast_from(0.25158473831683321654e-2_f64) * t30162 - t32377 - t32379 + t32380 + F::cast_from(0.37737710747524982482e-2_f64) * t30185 - F::cast_from(0.80031500487063509014e-2_f64) * t30187 + t32384 - t32385 - t32386 - t32387 - F::cast_from(0.94344276868812456205e-2_f64) * t30201 + F::cast_from(0.64025200389650807212e-1_f64) * t30203 - F::cast_from(0.25158473831683321656e-2_f64) * t30212 - F::cast_from(0.5590771962596293701e-2_f64) * t30217 + F::cast_from(0.42874018118069736972e-2_f64) * t30220 - t36870 - F::cast_from(0.12862205435420921092e-1_f64) * t33947;
    t36872
}
