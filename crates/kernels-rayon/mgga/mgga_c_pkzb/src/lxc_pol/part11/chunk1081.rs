//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1081/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1081(t148: f64, t779: f64, t179: f64, t299: f64, t655: f64, t5722: f64, t768: f64, t46: f64, t5953: f64, t5719: f64, t5932: f64, t2003: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18107 = t148 * t779;
    let t18110 = t299 * t179 * t18107 * t655;
    let t18152 = t768 * t5722;
    let t18153 = t18152 * t46;
    let t18154 = t5953 * t18153;
    let t18160 = t5719 * t18153;
    let t18163 = t5932 * t18153;
    let t18182 = t67 * t2003;
    (t18107, t18110, t18154, t18160, t18163, t18182)
}
