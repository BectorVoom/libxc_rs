//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1118/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1118(t13738: f64, t8676: f64, t21: f64, t3142: f64, t3712: f64, t8654: f64, t4043: f64, t1030: f64, t26312: f64, t20487: f64, t3141: f64, t3131: f64, t3137: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t26369 = t8676 * t13738;
    let t26396 = t3712 * t3142 * t21;
    let t26416 = t8654 * pi;
    let t26447 = t4043 * pi;
    let t26561 = t1030 * t26312;
    let t26578 = t3141 * t20487;
    let t26597 = t3131 * t3137;
    (t26369, t26396, t26416, t26447, t26561, t26578, t26597)
}
