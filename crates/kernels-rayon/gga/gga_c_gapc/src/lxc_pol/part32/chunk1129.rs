//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1129/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1129(t17760: f64, t2580: f64, t33273: f64, t1: f64, t33549: f64, t128: f64, t18639: f64, t941: f64, t2660: f64, t24759: f64, t667: f64, t277: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t33956 = t17760 * t33273 * t2580;
    let t33958 = t33549 * t1;
    let t33961 = t18639 * t941 * t128;
    let t33962 = t2660 * t33958 * t33961;
    let t33965 = t667 * t24759 * pi;
    let t33966 = t277 * t33965;
    (t33956, t33958, t33961, t33962, t33965, t33966)
}
