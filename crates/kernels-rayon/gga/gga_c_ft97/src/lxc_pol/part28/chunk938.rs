//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 938/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk938(t32145: f64, t92335: f64, t136307: f64, t420: f64, t173: f64, t32151: f64, t22796: f64, t5572: f64, t22581: f64, t32146: f64, t1691: f64, t32318: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t136356 = t92335 * t32145;
    let t136359 = t136307 * t420;
    let t136363 = t32151 * t173;
    let t136365 = t22796 * t136363 * t5572;
    let t136367 = t22581 * t173;
    let t136369 = t32146 * t136367 * t5572;
    let t136403 = t1691 * t32318;
    (t136356, t136359, t136363, t136365, t136367, t136369, t136403)
}
