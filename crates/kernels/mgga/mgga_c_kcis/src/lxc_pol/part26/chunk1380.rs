//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1380/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1380<F: Float>(t103662: F, t27339: F, t102655: F, t102658: F, t102661: F, t102666: F, t102669: F, t103069: F, t28388: F, t98830: F, t98845: F, t98849: F, t98854: F, t98864: F) -> F {
    let t103702 = t27339 * t103662;
    let t103712 = F::new(0.88437037037037037033e-2) * t102655 - F::new(0.61890573922526041667e-5) * t103702 + F::new(0.13265555555555555555e-1) * t102658 - F::new(0.88437037037037037033e-2) * t102661 - F::new(0.7369753086419753086e-3) * t98830 - F::new(0.37134344353515625e-4) * t28388 * t103069 + F::new(0.1621345679012345679e-1) * t102666 - F::new(0.92673611111111111112e-3) * t98845 - t98849 - t98854 + F::new(0.16581944444444444444e-2) * t102669 - t98864;
    t103712
}
