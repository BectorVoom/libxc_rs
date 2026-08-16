//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 620/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk620(t28023: f64, t766: f64, t24232: f64, t3875: f64, t24231: f64, t1425: f64, t683: f64, t2360: f64, t263: f64, t3886: f64, t2404: f64, t2347: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28024 = t28023 * t766;
    let t28026 = t24232 * t3875;
    let t28027 = t24231 * t28026;
    let t28030 = t683 * t1425;
    let t28031 = t263 * t2360;
    let t28032 = t28031 * t3886;
    let t28033 = t28030 * t28032;
    let t28036 = t2404 * t1425;
    let t28037 = t263 * t2347;
    (t28024, t28026, t28027, t28030, t28032, t28033, t28036, t28037)
}
