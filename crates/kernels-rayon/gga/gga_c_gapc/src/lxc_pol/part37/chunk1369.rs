//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1369/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1369(t33356: f64, t33358: f64, t33360: f64, t33364: f64, t33369: f64, t33371: f64, t33383: f64, t33387: f64, t33390: f64, t33394: f64, t33396: f64, t33402: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36559 = 0.10567613244746075633e-6_f64 * t33356;
    let t36560 = 0.2318836277704281739e-4_f64 * t33358;
    let t36561 = 0.2318836277704281739e-4_f64 * t33360;
    let t36562 = 0.71696352428860134552e-9_f64 * t33364;
    let t36563 = 0.94685814672924837674e-4_f64 * t33369;
    let t36564 = 0.21642471925239962898e-3_f64 * t33371;
    let t36568 = 0.33816362383187442026e-5_f64 * t33383;
    let t36570 = 0.77294542590142724634e-6_f64 * t33387;
    let t36571 = 0.67528199161846004232e-6_f64 * t33390;
    let t36572 = 0.99537768901660885081e-7_f64 * t33394;
    let t36573 = 0.10551281119038438161e-7_f64 * t33396;
    let t36574 = 0.44197102999375800018e-8_f64 * t33402;
    (t36559, t36560, t36561, t36562, t36563, t36564, t36568, t36570, t36571, t36572, t36573, t36574)
}
