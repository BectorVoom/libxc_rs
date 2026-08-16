//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 260/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk260(t1576: f64, t171: f64, t11: f64, t41: f64, t1136: f64, t2330: f64, t1087: f64, t2336: f64, t89: f64, t2347: f64, t992: f64, t505: f64) -> (f64, f64, f64, f64) {
    let t3626 = 1.0_f64 / t171 / t1576;
    let t3627 = t11 * t3626;
    let t3628 = t41 * t3627;
    let t3683 = t2330 * t1136;
    let t3688 = t89 * t2336 * t1087;
    let t3690 = t2347 * t992;
    let t3691 = t3690 * t505;
    (t3628, t3683, t3688, t3691)
}
