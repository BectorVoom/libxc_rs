//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1309/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1309<F: Float>(t140: F, t104684: F, t104740: F, t104793: F, t104841: F, t104885: F, t104928: F, t104970: F, t105010: F, t105057: F, t105099: F, t105148: F, t105182: F, t105218: F, t105255: F, t105293: F, t105324: F) -> (F,) {
    let t141 = 0.1e-59 < t140;
    let t105329 = piecewise3(t141, t104684 + t104740 + t104793 + t104841 + t104885 + t104928 + t104970 + t105010 + t105057 + t105099 + t105148 + t105182 + t105218 + t105255 + t105293 + t105324, 0.0);
    (t105329,)
}
