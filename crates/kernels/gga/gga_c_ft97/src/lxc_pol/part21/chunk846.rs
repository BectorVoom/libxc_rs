//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 846/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk846<F: Float>(t26061: F, t492: F, t25873: F, t25876: F, t25881: F, t25886: F, t25891: F, t25897: F, t25902: F, t25906: F, t25910: F, t25913: F, t25917: F, t25921: F, t22980: F, t22991: F, t23016: F, t23029: F, t23038: F, t25926: F, t25931: F, t25935: F, t25940: F, t25944: F, t25946: F, t25948: F) -> (F, F, F) {
    let t26062 = t26061 * t492;
    let t26077 = -3.0 * t25873 + t25876 / 6.0 + t25881 / 3.0 - t25886 / 2.0 - t25891 / 2.0 - 3.0 / 8.0 * t25897 + t25902 / 6.0 + 2.0 * t25906 + 2.0 * t25910 - 2.0 / 3.0 * t25913 + 2.0 * t25917 - t25921 / 3.0;
    let t26089 = -t25926 / 3.0 + t25931 / 9.0 - t25935 / 3.0 - t22980 / 3.0 - t22991 / 9.0 + t25940 / 3.0 + t25944 / 3.0 - t25946 / 9.0 - t25948 / 18.0 - t23016 / 12.0 + t23029 / 6.0 - t23038;
    (t26062, t26077, t26089)
}
