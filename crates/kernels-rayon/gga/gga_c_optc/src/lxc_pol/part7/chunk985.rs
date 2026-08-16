//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 985/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk985(t1113: f64, t23: f64, t3273: f64, t4280: f64, t24: f64, t3086: f64, t496: f64, t8414: f64, t8: f64, t465: f64, t8113: f64, t19: f64, t3126: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11781 = t23 * t1113;
    let t11786 = t3273 * t4280;
    let t11885 = t24 * t3086;
    let t11894 = t496 * t8414;
    let t11899 = t8 * t3086;
    let t11943 = t465 * t8113;
    let t11970 = t19 * t3126;
    (t11781, t11786, t11885, t11894, t11899, t11943, t11970)
}
