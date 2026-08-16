//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1024/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1024(t1871: f64, t22952: f64, t26016: f64, t5675: f64, t34384: f64, t379: f64, t22958: f64, t5674: f64, t136159: f64, t136188: f64, t25883: f64, t32069: f64) -> (f64, f64, f64, f64) {
    let t144892 = t22952 * t1871 * t5675 * t26016;
    let t144893 = t34384 * t379;
    let t144895 = t5674 * t22958 * t144893;
    let t144899 = t136159 * t136188 * t32069 * t25883;
    (t144892, t144893, t144895, t144899)
}
