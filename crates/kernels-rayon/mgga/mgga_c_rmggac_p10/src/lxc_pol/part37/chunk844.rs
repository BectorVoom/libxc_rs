//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 844/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk844(t2067: f64, t3369: f64, t70460: f64, t75154: f64, t15280: f64, t325: f64, t14170: f64, t14131: f64, t21714: f64, t9152: f64, t14117: f64, t69594: f64, t8416: f64) -> (f64, f64, f64, f64) {
    let t75157 = t70460 * t3369 * t2067 * t75154;
    let t75162 = t15280 * t325;
    let t75163 = t75162 * t14170;
    let t75166 = t14131 * t21714 * t9152;
    let t75169 = t69594 * t14117 * t8416;
    (t75157, t75163, t75166, t75169)
}
