//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1070/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1070(t1381: f64, t2353: f64, t501: f64, t8040: f64, t1959: f64, t2967: f64, t747: f64, t9032: f64, t1022: f64, t5501: f64, t835: f64, t8720: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24215 = t2353 * t1381;
    let t24282 = t8040 * t501;
    let t24295 = t2967 * t1959;
    let t24303 = t9032 * t747;
    let t24321 = t5501 * t1022;
    let t24339 = t835 * t8720;
    (t24215, t24282, t24295, t24303, t24321, t24339)
}
