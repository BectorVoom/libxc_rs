//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1091/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1091<F: Float>(t41613: F, t41619: F, t302: F, t36506: F, t36508: F, t36511: F, t36513: F, t36515: F, t37964: F, t41616: F, t41627: F, t41631: F, t41635: F, t41637: F, t41639: F, t41641: F, t4965: F, t72: F, t9427: F, t9595: F) -> F {
    let t43761 = F::new(0.60975299583150056624e-3) * t41613;
    let t43763 = F::new(0.60975299583150056624e-3) * t41619;
    let t43780 = -t43761 - F::new(0.1440846329149835838e-2) * t41616 - t43763 + F::new(2.0) * t72 * t302 * t9595 + F::new(0.212822999466489197e-4) * t41627 - F::new(0.5454932330849068346e-1) * t41631 - F::new(0.8182398496273602519e0) * t41635 - F::new(0.72732431077987577947e0) * t41637 - F::new(0.16364796992547205038e0) * t41639 + F::new(0.43639458646792546768e0) * t41641 + F::new(0.79828278012425390428e-1) * t4965 * t9427 + t37964 + F::new(0.19863479950205658386e-4) * t36506 - F::new(0.13242319966803772257e-3) * t36508 + F::new(0.39726959900411316772e-3) * t36511 - F::new(0.39726959900411316772e-3) * t36513 - F::new(0.13242319966803772257e-3) * t36515;
    t43780
}
