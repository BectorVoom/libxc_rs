//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1031/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1031(t12276: f64, t12325: f64, t224: f64, t1531: f64, t2876: f64, t2097: f64, t3039: f64, t123: f64, t3689: f64, t3720: f64, t5558: f64, t744: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12326 = t12276 + t12325;
    let t12327 = t224 * t12326;
    let t12881 = t2876 * t1531;
    let t13045 = t3039 * t2097;
    let t13777 = t3689 * t123;
    let t13846 = t3720 * t123;
    let t14537 = t744 * t5558;
    (t12326, t12327, t12881, t13045, t13777, t13846, t14537)
}
