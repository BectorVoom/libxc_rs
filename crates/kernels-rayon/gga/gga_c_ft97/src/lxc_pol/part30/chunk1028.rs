//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1028/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1028(t6789: f64, t694: f64, t1419: f64, t690: f64, t1418: f64, t150533: f64, t33372: f64, t3817: f64, t52: f64, t7457: f64, t108517: f64, t1410: f64, t22794: f64, t39: f64) -> (f64, f64, f64, f64, f64) {
    let t150618 = t694 * t6789;
    let t150621 = t1419 * t690;
    let t150625 = t33372 * t1418 * t150533;
    let t150630 = t52 * t7457 * t3817;
    let t150637 = t108517 * t1410 * t39 * t22794;
    (t150618, t150621, t150625, t150630, t150637)
}
