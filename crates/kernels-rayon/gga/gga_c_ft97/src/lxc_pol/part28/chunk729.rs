//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 729/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk729(t32068: f64, t32069: f64, t379: f64, t32067: f64, t7240: f64, t81: f64, t7242: f64, t432: f64, t7165: f64, t7238: f64, t7239: f64, t1307: f64, t5617: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32071 = t32068 * t32069 * t379;
    let t32072 = t32067 * t32071;
    let t32075 = 1.0_f64 / t7240 / t81;
    let t32076 = t32075 * t7242;
    let t32077 = t7165 * t432;
    let t32078 = t32076 * t32077;
    let t32080 = t7238 * t7239 * t32078;
    let t32082 = t1307 * t5617;
    (t32071, t32072, t32075, t32076, t32077, t32078, t32080, t32082)
}
