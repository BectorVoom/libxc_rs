//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 985/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk985(t3784: f64, t9865: f64, t11933: f64, t19: f64, t311: f64, t3752: f64, t3750: f64, t869: f64, t1453: f64, t3760: f64, t9555: f64, t190: f64, t6851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11951 = t3784 * t9865;
    let t11953 = t11933 * t19;
    let t11954 = t311 * t11953;
    let t11955 = t11954 * t3752;
    let t11957 = t869 * t3750;
    let t11958 = t11957 * t3752;
    let t11960 = t3760 * t1453;
    let t11961 = t311 * t11960;
    let t11962 = t11961 * t9555;
    let t11964 = t6851 * t190;
    (t11951, t11953, t11954, t11955, t11957, t11958, t11960, t11961, t11962, t11964)
}
