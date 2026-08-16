//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 861/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk861(t245: f64, t35306: f64, t35785: f64, t21: f64, t5: f64, t7565: f64, t920: f64, t33983: f64, t6970: f64, t193: f64, t33966: f64, t28985: f64, t6222: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t246 = 10000000.0_f64 <= t245;
    let t35786 = t35306 + t35785;
    let t35793 = piecewise3(t246, 0.0_f64, t5 * t35786 * t21 / 4.0_f64 + t5 * t7565 * t920 / 4.0_f64);
    let t35794 = t33983 * t6970;
    let t35795 = t193 * t35794;
    let t35798 = t33966 * t6970;
    let t35799 = t193 * t35798;
    let t35801 = t6222 * t28985;
    (t35786, t35793, t35794, t35795, t35798, t35799, t35801)
}
