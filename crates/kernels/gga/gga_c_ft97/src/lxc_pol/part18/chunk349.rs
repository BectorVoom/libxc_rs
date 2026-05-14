//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 349/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk349<F: Float>(t1970: F, t2102: F, t1792: F, t582: F, t1796: F, t1984: F, t2: F) -> (F, F, F, F) {
    let t2103 = t2102 * t1970;
    let t2106 = t582 * t1792;
    let t2109 = t582 * t1796;
    let t2112 = t1984 * t2;
    (t2103, t2106, t2109, t2112)
}
