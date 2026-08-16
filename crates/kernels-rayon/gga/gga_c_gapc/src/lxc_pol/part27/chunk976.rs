//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 976/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk976(t11822: f64, t2664: f64, t11513: f64, t2660: f64, t3696: f64, t3781: f64, t761: f64, t11557: f64, t3780: f64, t11736: f64, t277: f64, t612: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11823 = t11822 * t2664;
    let t11825 = t2660 * t11513;
    let t11826 = t11825 * t2664;
    let t11829 = t761 * t3696 * t3781;
    let t11831 = t11557 * t3780;
    let t11832 = t11736 * t11831;
    let t11834 = t277 * t612;
    (t11823, t11825, t11826, t11829, t11831, t11832, t11834)
}
