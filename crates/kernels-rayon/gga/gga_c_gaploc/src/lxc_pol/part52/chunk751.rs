//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 751/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk751(t26126: f64, t544: f64, t18535: f64, t19: f64, t584: f64, t60: f64, t18540: f64, t201: f64, t1397: f64, t8410: f64, t1: f64, t106: f64, t4524: f64) -> (f64, f64, f64, f64, f64) {
    let t34286 = t544 * t26126;
    let t34400 = t584 * t18535 * t19 * t60;
    let t34401 = t201 * t18540;
    let t34471 = t1397 * t8410;
    let t34506 = t544 * t4524 * t1 * t106;
    (t34286, t34400, t34401, t34471, t34506)
}
