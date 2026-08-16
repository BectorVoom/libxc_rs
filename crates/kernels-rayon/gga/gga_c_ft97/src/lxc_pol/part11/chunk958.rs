//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 958/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk958(t2035: f64, t2037: f64, t2059: f64, t126: f64, t37627: f64, t120: f64, t37640: f64, t7977: f64, t32: f64, t7911: f64, t8991: f64, t123: f64, t37993: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39854 = t2035 * t2037 * t2059;
    let t39861 = t37627 * t126;
    let t39866 = t37640 * t120;
    let t39869 = t120 * t7977;
    let t39872 = t37640 * t126;
    let t39877 = t8991 / t32 / t7911;
    let t39889 = t123 / t532 / t37993;
    (t39854, t39861, t39866, t39869, t39872, t39877, t39889)
}
