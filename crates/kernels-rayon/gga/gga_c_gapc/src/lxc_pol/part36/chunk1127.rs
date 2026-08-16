//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1127/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1127(t15699: f64, t7502: f64, t9895: f64, t15680: f64, t26597: f64, t7259: f64, t11986: f64, t3367: f64, t6182: f64, t2268: f64, t3438: f64, t3439: f64) -> (f64, f64, f64, f64) {
    let t33917 = t9895 * t7502 * t15699;
    let t33920 = t7259 * t26597 * t15680;
    let t33923 = t11986 * t3367 * t6182;
    let t33928 = t3438 * t2268 * t3439;
    (t33917, t33920, t33923, t33928)
}
