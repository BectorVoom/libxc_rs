//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1028/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1028<F: Float>(t6789: F, t694: F, t1419: F, t690: F, t1418: F, t150533: F, t33372: F, t3817: F, t52: F, t7457: F, t108517: F, t1410: F, t22794: F, t39: F) -> (F, F, F, F, F) {
    let t150618 = t694 * t6789;
    let t150621 = t1419 * t690;
    let t150625 = t33372 * t1418 * t150533;
    let t150630 = t52 * t7457 * t3817;
    let t150637 = t108517 * t1410 * t39 * t22794;
    (t150618, t150621, t150625, t150630, t150637)
}
