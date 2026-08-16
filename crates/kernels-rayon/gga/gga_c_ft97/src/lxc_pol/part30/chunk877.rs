//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 877/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk877(t193: f64, t36012: f64, t1253: f64, t7612: f64, t34031: f64, t34036: f64, t35822: f64, t35826: f64, t35831: f64, t35836: f64, t35840: f64, t35844: f64, t35848: f64, t35851: f64, t35856: f64) -> (f64, f64, f64, f64) {
    let t36013 = t193 * t36012;
    let t36016 = t7612 * t1253;
    let t36017 = t193 * t36016;
    let t36033 = 3.0_f64 / 2.0_f64 * t35822 + t34031 + 2.0_f64 / 3.0_f64 * t35826 + 4.0_f64 * t35831 - 2.0_f64 * t35836 - t35840 / 2.0_f64 - t34036 - t35844 / 3.0_f64 - 3.0_f64 * t35848 + 2.0_f64 * t35851 + t35856 / 4.0_f64;
    (t36013, t36016, t36017, t36033)
}
