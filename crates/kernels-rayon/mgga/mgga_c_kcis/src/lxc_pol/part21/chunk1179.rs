//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1179/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1179(t684: f64, t9261: f64, t20: f64, t4879: f64, t14758: f64, t2840: f64, t4992: f64, t86: f64, t1773: f64, t9545: f64, t1709: f64, t31297: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37013 = t684 * t9261;
    let t37041 = t4879 * t20;
    let t42385 = t14758 * sigma0;
    let t42530 = t86 * t4992 * t2840;
    let t42570 = t9545 * t1773;
    let t42625 = t1709 * t31297;
    (t37013, t37041, t42385, t42530, t42570, t42625)
}
