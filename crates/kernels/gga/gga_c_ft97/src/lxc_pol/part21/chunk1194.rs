//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1194/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1194<F: Float>(t117054: F, t117067: F, t117079: F, t117090: F, t117102: F, t117111: F, t117123: F, t117133: F, t117145: F, t117154: F, t117166: F, t117178: F, t117190: F, t117199: F, t117211: F, t117222: F) -> (F,) {
    let t117226 = t117054 + t117067 + t117079 + t117090 + t117102 + t117111 + t117123 + t117133 + t117145 + t117154 + t117166 + t117178 + t117190 + t117199 + t117211 + t117222;
    (t117226,)
}
