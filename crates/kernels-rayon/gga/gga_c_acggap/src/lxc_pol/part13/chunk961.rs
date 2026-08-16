//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 961/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk961(t31878: f64, t3453: f64, t2131: f64, t2132: f64, t309: f64, t7877: f64, t7980: f64, t7987: f64, t1264: f64, t2138: f64, t2139: f64, t2147: f64) -> (f64, f64, f64, f64) {
    let t31879 = t31878 * t3453;
    let t31895 = t2131 * t2132 * t7877 * t309;
    let t31897 = t7987 * t7980;
    let t31901 = t2138 * t2147 * t2139 * t1264;
    (t31879, t31895, t31897, t31901)
}
