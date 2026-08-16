//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1070/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1070(t2288: f64, t8406: f64, t15386: f64, t31057: f64, t2297: f64, t8906: f64, t31195: f64, t13287: f64, t8960: f64, t17912: f64, t31443: f64, t5616: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38857 = t2288 * t8406;
    let t38859 = t31057 * t15386 * t38857;
    let t38861 = t2297 * t8906;
    let t38863 = t31195 * t15386 * t38861;
    let t38867 = t31195 * t13287 * t2297 * t8960;
    let t38871 = t31443 * t17912 * t2288 * t5616;
    (t38857, t38859, t38861, t38863, t38867, t38871)
}
