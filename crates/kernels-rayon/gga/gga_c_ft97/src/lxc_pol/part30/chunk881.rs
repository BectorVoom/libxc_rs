//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 881/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk881(t1506: f64, t7022: f64, t193: f64, t1253: f64, t7585: f64, t1248: f64, t7679: f64, t2843: f64, t7672: f64, t10697: f64, t25188: f64, t7114: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36048 = t7022 * t1506;
    let t36049 = t193 * t36048;
    let t36056 = t7585 * t1253;
    let t36057 = t193 * t36056;
    let t36060 = t7679 * t1248;
    let t36061 = t2843 * t36060;
    let t36063 = t7672 * t1248;
    let t36064 = t10697 * t36063;
    let t36066 = t25188 * t7114;
    (t36048, t36049, t36056, t36057, t36060, t36061, t36063, t36064, t36066)
}
