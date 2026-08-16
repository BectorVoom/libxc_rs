//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 843/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk843(t8786: f64, t9894: f64, t9897: f64, t3120: f64, t3402: f64, t1086: f64, t2628: f64, t2233: f64, t2982: f64, t3387: f64, t3138: f64, t3363: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10043 = t9894 * t8786;
    let t10044 = t10043 * t9897;
    let t10046 = t3402 * t3120;
    let t10047 = t1086 * t2628;
    let t10048 = t10046 * t10047;
    let t10050 = t2982 * t2233;
    let t10051 = t3387 * t10050;
    let t10053 = t3363 * t3138;
    (t10043, t10044, t10047, t10048, t10051, t10053)
}
