//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 802/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk802(t21708: f64, t21716: f64, t241: f64, t258: f64, t21369: f64, t265: f64, t724: f64, t10024: f64, t21351: f64, t1091: f64, t5181: f64, t1175: f64, t4973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21717 = t21708 + t21716;
    let t21719 = t241 * t21717 * t258;
    let t21724 = t724 * t265 * t21369;
    let t21728 = t10024 * t265 * t21351;
    let t21732 = t724 * t5181 * t1091;
    let t21736 = t724 * t1175 * t4973;
    (t21717, t21719, t21724, t21728, t21732, t21736)
}
