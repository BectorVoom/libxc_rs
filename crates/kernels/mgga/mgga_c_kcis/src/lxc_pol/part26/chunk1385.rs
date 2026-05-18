//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1385/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1385<F: Float>(t103095: F, t103132: F, t103154: F, t103185: F, t103209: F, t103233: F, t103268: F, t103292: F, t103318: F, t103347: F, t103366: F, t103402: F, t103418: F, t103438: F, t103459: F, t103475: F, t103494: F, t103527: F, t103551: F, t103572: F, t103586: F, t103608: F, t103624: F, t103649: F, t103669: F, t103686: F, t103700: F, t103712: F, t103731: F, t103747: F, t103762: F, t103779: F, t589: F) -> F {
    let t103785 = (t103475 + t103624 + t103572 + t103438 + t103402 + t103154 + t103586 + t103459 + t103233 + t103209 + t103700 + t103608 + t103712 + t103418 + t103366 + t103686 + t103268 + t103132 + t103731 + t103318 + t103292 + t103095 + t103669 + t103649 + t103494 + t103779 + t103747 + t103527 + t103762 + t103551 + t103185 + t103347) * t589;
    t103785
}
