//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1159/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1159(t33940: f64, t30157: f64, t30162: f64, t30185: f64, t30187: f64, t30201: f64, t30203: f64, t30212: f64, t30217: f64, t30220: f64, t32377: f64, t32379: f64, t32380: f64, t32384: f64, t32385: f64, t32386: f64, t32387: f64, t33947: f64) -> f64 {
    let t36870 = 0.21437009059034868486e-2_f64 * t33940;
    let t36872 = -0.62896184579208304138e-2_f64 * t30157 + 0.25158473831683321654e-2_f64 * t30162 - t32377 - t32379 + t32380 + 0.37737710747524982482e-2_f64 * t30185 - 0.80031500487063509014e-2_f64 * t30187 + t32384 - t32385 - t32386 - t32387 - 0.94344276868812456205e-2_f64 * t30201 + 0.64025200389650807212e-1_f64 * t30203 - 0.25158473831683321656e-2_f64 * t30212 - 0.5590771962596293701e-2_f64 * t30217 + 0.42874018118069736972e-2_f64 * t30220 - t36870 - 0.12862205435420921092e-1_f64 * t33947;
    t36872
}
