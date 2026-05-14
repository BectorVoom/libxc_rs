//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1115/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1115<F: Float>(t1349: F, t24069: F, t376: F, t23402: F, t24073: F, t5766: F, t24094: F, t1361: F, t7943: F, t23925: F, t378: F, t23405: F, t24083: F, t1359: F, t2228: F, t24087: F) -> (F, F, F, F, F, F, F, F, F) {
    let t94206 = t1349 * t376 * t24069;
    let t94214 = t1349 * t376 * t23402;
    let t94215 = t5766 * t24073;
    let t94217 = t5766 * t24094;
    let t94227 = 14.0 / 81.0 * t1349 * t7943 * t1361;
    let t94230 = t378 * t23925;
    let t94234 = t23405 * t24083;
    let t94251 = t1359 * t2228;
    let t94258 = t5766 * t24087;
    (t94206, t94214, t94215, t94217, t94227, t94230, t94234, t94251, t94258)
}
