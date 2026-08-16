//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1117/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1117(t10079: f64, t11430: f64, t3363: f64, t11869: f64, t29861: f64, t11522: f64, t18551: f64, t30288: f64, t11924: f64, t29582: f64, t11910: f64, t30325: f64) -> (f64, f64, f64, f64, f64) {
    let t33863 = t3363 * t11430 * t10079;
    let t33865 = t11869 * t29861;
    let t33868 = t18551 * t11522 * t30288;
    let t33870 = t11924 * t29582;
    let t33872 = t11910 * t30325;
    (t33863, t33865, t33868, t33870, t33872)
}
