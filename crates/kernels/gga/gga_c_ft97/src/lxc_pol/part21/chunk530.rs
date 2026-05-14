//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 530/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk530<F: Float>(t2179: F, t6708: F, t144: F, t5962: F, t5965: F, t6659: F, t6663: F, t6667: F, t6671: F, t6675: F, t6679: F, t6683: F) -> (F, F, F) {
    let t6709 = t2179 * t6708;
    let t6710 = t144 * t6709;
    let t6718 = t6659 / 4.0 + t5962 + t6663 / 6.0 + t6667 - t6671 / 2.0 + t5965 + t6675 / 3.0 + 2.0 * t6679 - t6683;
    (t6709, t6710, t6718)
}
