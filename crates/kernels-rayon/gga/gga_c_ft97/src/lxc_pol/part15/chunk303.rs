//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 303/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk303(t407: f64, t76: f64, t66: f64, t47: f64, t625: f64, t68: f64, t72: f64, t23: f64, t358: f64) -> (f64, f64, f64, f64, f64) {
    let t1710 = 1.0_f64 / t407 / t76;
    let t1711 = t66 * t1710;
    let t1728 = t47 * t625;
    let t1730 = t68 * t1728 * t72;
    let t1731 = 0.42562405586419753087e-2_f64 * t1730;
    let t1736 = 1.0_f64 / t23 / t358;
    (t1710, t1711, t1730, t1731, t1736)
}
