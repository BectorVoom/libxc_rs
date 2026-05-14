//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1026/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1026<F: Float>(t1293: F, t1711: F, t388: F, t1602: F, t92488: F, t37481: F, t5551: F, t5555: F, t1611: F, t58: F, t1620: F, t32250: F, t5537: F, t5547: F, t22602: F, t5560: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t92685 = t1711 * t1293;
    let t92686 = t388 * t92685;
    let t92689 = t1602 * t92488;
    let t92710 = t37481 * t5551 * t5555;
    let t92715 = t1611 * sigma0 * t58;
    let t92770 = t32250 * t1620;
    let t92773 = t5537 * t5547;
    let t92776 = t22602 * t5560;
    (t92685, t92686, t92689, t92710, t92715, t92770, t92773, t92776)
}
