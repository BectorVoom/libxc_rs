//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 808/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk808<F: Float>(t8876: F, t8879: F, t8882: F, t8898: F, t7686: F, t7699: F, t7710: F, t7714: F, t7722: F, t8235: F, t8240: F, t8247: F, t8249: F, t8885: F, t8890: F) -> F {
    let t9328 = t8876 / F::new(32.0);
    let t9329 = t8879 / F::new(96.0);
    let t9331 = F::new(0.5603125e-1) * t8882;
    let t9335 = F::new(0.21437009059034868486e-3) * t8898;
    let t9336 = -t9328 - t9329 + t8235 + F::new(0.40015750243531754507e-2) * t7686 - t8240 - t9331 + t8885 / F::new(24.0) + t8890 / F::new(24.0) - t7699 + F::new(0.62896184579208304137e-3) * t7710 - t7714 - t8247 - t7722 - t8249 + t9335;
    t9336
}
