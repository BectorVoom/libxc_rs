//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 574/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk574(t3451: f64, t4925: f64, t1541: f64, t3372: f64, t1005: f64, t1352: f64, t1588: f64, t997: f64, t3237: f64, t542: f64, t1581: f64, t537: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4926 = t3451 * t4925;
    let t4928 = t3372 * t1541;
    let t4946 = t1005 * t1352;
    let t4949 = 0.40015750243531754508e-2_f64 * t997 * t1588;
    let t4950 = t3237 * t542;
    let t4953 = 0.40015750243531754508e-2_f64 * t997 * t1581;
    let t4954 = t3237 * t537;
    (t4926, t4928, t4946, t4949, t4950, t4953, t4954)
}
