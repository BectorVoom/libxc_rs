//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1279/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1279(t1966: f64, t33760: f64, t590: f64, t28878: f64, t28880: f64, t2714: f64, t8634: f64, t2718: f64, t24817: f64, t955: f64, t14626: f64, t2087: f64, t3503: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33763 = 0.51123901271894332902e1_f64 * t1966 * t33760 * t590;
    let t33773 = 0.12780975317973583226e0_f64 * t28878;
    let t33774 = 0.63904876589867916128e-1_f64 * t28880;
    let t33786 = 0.71500979903700853338e0_f64 * t2714 * t8634;
    let t33788 = 0.71500979903700853338e0_f64 * t2718 * t8634;
    let t33790 = 0.35750489951850426669e0_f64 * t955 * t24817;
    let t33799 = 0.30674340763136599741e1_f64 * t2087 * t14626 * t3503;
    (t33763, t33773, t33774, t33786, t33788, t33790, t33799)
}
