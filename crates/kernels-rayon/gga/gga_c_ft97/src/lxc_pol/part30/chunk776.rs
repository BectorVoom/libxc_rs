//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 776/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk776(t7584: f64, t856: f64, t7641: f64, t33811: f64, t7512: f64, t33288: f64, t7638: f64, t7642: f64, t6307: f64, t631: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33812 = t7584 * t856;
    let t33813 = t7641 * t33812;
    let t33815 = t33811 * t7512 * t33813;
    let t33818 = t7638 * t33288 * t7642;
    let t33819 = 2.0_f64 / 9.0_f64 * t33818;
    let t33820 = t6307 * t631;
    (t33812, t33813, t33815, t33818, t33819, t33820)
}
