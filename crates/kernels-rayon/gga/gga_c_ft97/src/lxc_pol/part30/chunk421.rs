//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 421/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk421(t6940: f64, t762: f64, t242: f64, t1901: f64, t193: f64, t446: f64, t6073: f64, t6099: f64, t6160: f64, t6849: f64, t6854: f64, t6858: f64, t6863: f64, t6867: f64, t6871: f64, t6875: f64, t6909: f64, t6914: f64, t6918: f64, t6923: f64, t6927: f64, t6932: f64, t89: f64) -> (f64, f64) {
    let t6941 = t762 * t6940;
    let t6942 = t242 * t6941;
    let t6945 = t6073 + t1901 * t6849 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t6854 - t446 * t6858 / 3.0_f64 + t446 * t6863 / 3.0_f64 - t446 * t6867 / 3.0_f64 - t6099 - t446 * t6871 / 9.0_f64 - t446 * t6875 / 3.0_f64 + t89 * t193 * t6909 / 3.0_f64 - t446 * t6914 / 3.0_f64 + t6160 + t1901 * t6918 / 9.0_f64 + t446 * t6923 / 3.0_f64 - t446 * t6927 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t6932 - t446 * t6942 / 3.0_f64;
    (t6942, t6945)
}
