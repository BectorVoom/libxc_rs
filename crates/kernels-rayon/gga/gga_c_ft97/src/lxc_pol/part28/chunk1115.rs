//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1115/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1115(t140: f64, t147273: f64, t147319: f64, t147357: f64, t147402: f64, t147448: f64, t147492: f64, t147541: f64, t147586: f64, t138537: f64, t6584: f64, t32748: f64, t6580: f64) -> (f64, f64, f64) {
    let t141 = 0.1e-59_f64 < t140;
    let t147590 = piecewise3(t141, t147273 + t147319 + t147357 + t147402 + t147448 + t147492 + t147541 + t147586, 0.0_f64);
    let t147602 = t138537 * t6584;
    let t147604 = t6580 * t32748;
    (t147590, t147602, t147604)
}
