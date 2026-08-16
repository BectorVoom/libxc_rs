//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1019/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1019(t22952: f64, t22953: f64, t34405: f64, t379: f64, t136151: f64, t136159: f64, t144796: f64, t32067: f64, t32069: f64, t3266: f64, t36450: f64, t637: f64) -> (f64, f64, f64) {
    let t144829 = t22952 * t22953 * t34405 * t379;
    let t144832 = t136159 * t136151 * t144796;
    let t144836 = t32067 * t637 * t36450 * t32069 * t3266;
    (t144829, t144832, t144836)
}
