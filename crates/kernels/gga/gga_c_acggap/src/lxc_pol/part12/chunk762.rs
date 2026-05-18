//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 762/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk762<F: Float>(t7387: F, t7390: F, t7394: F, t7396: F, t7398: F, t7403: F, t7405: F, t7407: F, t7409: F, t7411: F, t7416: F, t7420: F, t7424: F, t7429: F, t7434: F, t7438: F, t7441: F, t7445: F, t7448: F, t7453: F) -> F {
    let t8169 = -t7387 / F::new(48.0) - F::new(0.305625e-1) * t7390 + t7394 / F::new(96.0) + F::new(0.5603125e-1) * t7396 + t7398 / F::new(24.0) + t7403 / F::new(16.0) + F::new(7.0) / F::new(72.0) * t7405 - t7407 / F::new(12.0) - t7409 / F::new(24.0) - t7411 / F::new(24.0) - F::new(0.62896184579208304138e-3) * t7416 + F::new(0.31448092289604152069e-3) * t7420 - F::new(0.21437009059034868486e-3) * t7424 - F::new(0.18868855373762491241e-2) * t7429 - F::new(0.37737710747524982482e-2) * t7434 + t7438 / F::new(12.0) - F::new(0.1120625e0) * t7441 - F::new(0.4584375e-1) * t7445 - F::new(0.16809375e0) * t7448 - F::new(0.916875e-1) * t7453;
    t8169
}
