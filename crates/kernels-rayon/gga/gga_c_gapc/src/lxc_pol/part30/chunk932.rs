//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 932/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk932(t1: f64, t11341: f64, t169: f64, t2974: f64, t8676: f64, t3669: f64, t561: f64, t1023: f64, t3663: f64, t563: f64, t2983: f64, t3684: f64, t659: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11342 = t11341 * t1;
    let t11343 = t169 * t11342;
    let t11344 = t8676 * t2974;
    let t11345 = t11343 * t11344;
    let t11347 = t561 * t3669;
    let t11348 = t11347 * t1023;
    let t11350 = t563 * t3663;
    let t11351 = t11350 * t2983;
    let t11353 = t3684 * t659;
    (t11342, t11344, t11345, t11347, t11348, t11350, t11351, t11353)
}
