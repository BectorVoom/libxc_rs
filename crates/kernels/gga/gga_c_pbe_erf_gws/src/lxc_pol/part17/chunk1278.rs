//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1278/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1278<F: Float>(t2408: F, t3212: F, t51084: F, t51540: F, t51667: F, t51683: F, t51688: F, t53748: F, t53750: F, t53751: F, t53758: F, t53761: F, t53768: F, t53772: F, t53775: F, t53779: F, t53784: F, t6793: F, t8629: F, t9283: F) -> F {
    let t53787 = -t53748 / F::new(384.0) - t53750 + t53751 / F::new(96.0) - t2408 * t9283 * t51084 * t3212 / F::new(12.0) - F::new(7.0) / F::new(576.0) * t51667 + t53758 / F::new(96.0) + t6793 * t53761 / F::new(24.0) + t8629 * t51540 / F::new(48.0) - t53768 / F::new(3072.0) - F::new(7.0) / F::new(48.0) * t51683 - F::new(7.0) / F::new(288.0) * t51688 - t53772 / F::new(96.0) - t53775 / F::new(48.0) - t6793 * t53779 / F::new(12.0) - t6793 * t53784 / F::new(8.0);
    t53787
}
