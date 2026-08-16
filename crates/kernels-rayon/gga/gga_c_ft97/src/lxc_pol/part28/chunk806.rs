//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 806/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk806(t32937: f64, t586: f64, t28: f64, t5890: f64, t32892: f64, t32896: f64, t32902: f64, t32910: f64, t32915: f64, t32919: f64, t32923: f64, t32927: f64, t32931: f64, t32935: f64) -> (f64, f64, f64) {
    let t32938 = t586 * t32937;
    let t32940 = t5890 * t28 * t32938;
    let t32942 = 3.0_f64 / 2.0_f64 * t32892 + t32896 + 2.0_f64 / 3.0_f64 * t32902 + 4.0_f64 * t32910 - 2.0_f64 * t32915 - t32919 / 2.0_f64 - t32923 - t32927 / 3.0_f64 - 3.0_f64 * t32931 + 2.0_f64 * t32935 + t32940 / 4.0_f64;
    (t32938, t32940, t32942)
}
