//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1158/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1158(t35151: f64, t376: f64, t89: f64, t1053: f64, t139173: f64, t26590: f64, t5968: f64, t106551: f64, t12277: f64, t140237: f64, t140239: f64, t140241: f64, t140253: f64, t140263: f64, t140268: f64, t140274: f64, t143: f64, t144: f64, t148686: f64, t148692: f64, t148703: f64, t160: f64, t1901: f64, t26836: f64, t26883: f64, t28: f64, t33060: f64, t34947: f64, t35229: f64, t3578: f64, t446: f64, t558: f64, t574: f64, t5935: f64, t5943: f64, t605: f64, t7357: f64) -> (f64, f64, f64) {
    let t148715 = t89 * t376 * t35151;
    let t148722 = t139173 * t1053;
    let t148726 = t26590 * t5968;
    let t148730 = 2.0_f64 / 9.0_f64 * t1901 * t106551 * t5943 + 2.0_f64 / 3.0_f64 * t446 * t574 * t5935 * t26883 + 2.0_f64 / 3.0_f64 * t446 * t574 * t5935 * t26836 + t140237 / 9.0_f64 + t140239 / 9.0_f64 + t89 * t28 * t143 * t148686 * t160 / 3.0_f64 + t148692 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t140241 + 2.0_f64 / 3.0_f64 * t446 * t574 * t12277 * t7357 + 2.0_f64 / 3.0_f64 * t446 * t574 * t3578 * t33060 - 2.0_f64 / 3.0_f64 * t446 * t144 * t148703 + 2.0_f64 / 9.0_f64 * t140253 + 2.0_f64 / 9.0_f64 * t140263 + t446 * t574 * t605 * t34947 * t558 / 3.0_f64 - t148715 / 9.0_f64 - t446 * t574 * t35229 * t558 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t140268 + t140274 - t446 * t144 * t148722 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t144 * t148726;
    (t148722, t148726, t148730)
}
