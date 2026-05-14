//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 561/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk561<F: Float>(t7105: F, t840: F, t871: F, t1501: F, t4246: F, t296: F, t1248: F) -> (F, F, F, F) {
    let t7107 = t840 * t871 * t7105;
    let t7110 = t4246 * t1501;
    let t7111 = t296 * t7110;
    let t7114 = t1501 * t1248;
    (t7107, t7110, t7111, t7114)
}
