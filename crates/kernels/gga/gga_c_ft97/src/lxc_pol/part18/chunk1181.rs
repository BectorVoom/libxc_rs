//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1181/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1181<F: Float>(t5569: F, t6441: F, t92652: F, t22582: F, t37835: F, t22535: F, t420: F, t22798: F, t3366: F, t8042: F, t92896: F, t22563: F, t3056: F, t7983: F, t25657: F, t7878: F) -> (F, F, F, F, F, F, F) {
    let t101173 = t5569 * t92652 * t6441;
    let t101193 = t37835 * t22582;
    let t101200 = t420 * t22535;
    let t101201 = t3366 * t22798;
    let t101209 = t8042 * t92896;
    let t101228 = t7983 * t22563 * t3056;
    let t101234 = t25657 * t7878;
    (t101173, t101193, t101200, t101201, t101209, t101228, t101234)
}
