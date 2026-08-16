//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 934/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk934(t10007: f64, t18446: f64, t1882: f64, t5066: f64, t13959: f64, t13961: f64, t13963: f64, t13965: f64, t14018: f64, t14020: f64, t14052: f64, t18431: f64, t18434: f64, t18439: f64, t18443: f64, t1901: f64, t9822: f64, t9824: f64) -> f64 {
    let t18447 = t10007 * t18446;
    let t18452 = t1882 * t5066;
    let t18454 = -t13959 - t13961 - t13963 + t13965 - t14018 - t14020 + t18431 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t1901 * t18434 - 4.0_f64 / 9.0_f64 * t1901 * t18439 + 4.0_f64 / 27.0_f64 * t1901 * t18443 - 2.0_f64 / 9.0_f64 * t1901 * t18447 - 4.0_f64 / 27.0_f64 * t9822 - 4.0_f64 / 27.0_f64 * t9824 - t14052 - 2.0_f64 / 9.0_f64 * t18452;
    t18454
}
