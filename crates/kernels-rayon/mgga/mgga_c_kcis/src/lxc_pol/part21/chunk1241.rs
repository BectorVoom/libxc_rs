//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1241/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1241(t10470: f64, t2180: f64, t27077: f64, t92751: f64, t3245: f64, t7732: f64, t1014: f64, t26720: f64, t26800: f64, t2822: f64, t26992: f64, t3500: f64, t7788: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93157 = t10470 * t2180;
    let t93158 = 0.51588271604938271604e-3_f64 * t93157;
    let t93161 = t27077 * t92751;
    let t93163 = t3245 * t7732;
    let t93171 = t1014 * t26720;
    let t93173 = t2822 * t26800;
    let t93196 = t7788 * t3500 * t26992;
    (t93157, t93158, t93161, t93163, t93171, t93173, t93196)
}
