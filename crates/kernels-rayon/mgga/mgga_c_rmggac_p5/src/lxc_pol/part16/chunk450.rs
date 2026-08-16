//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 450/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk450(t107: f64, t622: f64, t1656: f64, t290: f64, t1587: f64, t338: f64, t1614: f64, t321: f64, t570: f64) -> (f64, f64, f64, f64, f64) {
    let t5058 = t622 * t107;
    let t5061 = t290 * t1656;
    let t5098 = t338 * t1587;
    let t5126 = t338 * t1614;
    let t5144 = t570 * t321;
    (t5058, t5061, t5098, t5126, t5144)
}
