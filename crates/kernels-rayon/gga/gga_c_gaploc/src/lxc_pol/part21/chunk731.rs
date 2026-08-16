//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 731/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk731(t4598: f64, t917: f64, t1628: f64, t2433: f64, t569: f64, t6393: f64, t568: f64, t1265: f64, t161: f64, t165: f64) -> (f64, f64, f64, f64) {
    let t6876 = t4598 * t917;
    let t6881 = t1628 * t2433;
    let t6888 = t569 * t6393;
    let t6889 = t568 * t6888;
    let t6895 = t161 * t165 * t1265;
    (t6876, t6881, t6889, t6895)
}
