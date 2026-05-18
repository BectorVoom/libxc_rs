//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1074/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1074<F: Float>(t40458: F, t35707: F, t35720: F, t35724: F, t35731: F, t35737: F, t37821: F, t37822: F, t37825: F, t40420: F, t40425: F, t40431: F, t40437: F, t40442: F, t40448: F, t40451: F, t40456: F) -> F {
    let t43422 = F::new(0.15965655602485078085e0) * t40458;
    let t43428 = F::new(0.1064114997332445985e-4) * t40420 + F::new(0.1064114997332445985e-4) * t40425 - F::new(0.1702583995731913576e-4) * t40431 - F::new(0.1702583995731913576e-4) * t40437 + F::new(0.5107751987195740728e-4) * t40442 - F::new(0.5107751987195740728e-4) * t40448 + F::new(0.1702583995731913576e-4) * t40451 - F::new(0.212822999466489197e-4) * t40456 - t43422 + F::new(0.12195059916630011325e-2) * t35707 + t37821 + t37822 - F::new(0.17347588262831798123e-3) * t35720 - F::new(0.17347588262831798123e-3) * t35724 - t37825 - F::new(0.60975299583150056624e-3) * t35731 - F::new(0.30487649791575028312e-3) * t35737;
    t43428
}
