//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1124/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1124(t2645: f64, t3769: f64, t11905: f64, t15473: f64, t11822: f64, t7522: f64, t11825: f64, t17891: f64, t17899: f64, t26416: f64, t291: f64, t5542: f64) -> (f64, f64, f64, f64, f64) {
    let t33836 = t3769 * t2645;
    let t33838 = t11905 * t15473;
    let t33840 = t11822 * t7522;
    let t33842 = t11825 * t7522;
    let t33847 = t17891 * t5542 * t26416 * t291 * t17899;
    (t33836, t33838, t33840, t33842, t33847)
}
