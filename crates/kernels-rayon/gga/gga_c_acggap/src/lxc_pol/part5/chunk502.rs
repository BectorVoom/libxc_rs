//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 502/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk502(t1670: f64, t495: f64, t326: f64, t38: f64, t19: f64, t420: f64, t128: f64, t130: f64, t163: f64, t167: f64, t228: f64, t577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1946 = t1670 * t495;
    let t1963 = t38 * t326;
    let t1964 = 1.0_f64 / t1963;
    let t1981 = t420 * t19;
    let t1982 = t1981 * t128;
    let t2015 = t130 * t163;
    let t2028 = t167 * t19;
    let t2029 = t2028 * t128;
    let t2035 = t130 * t228;
    let t2059 = t577 * t128;
    (t1946, t1963, t1964, t1982, t2015, t2029, t2035, t2059)
}
