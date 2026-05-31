//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1158/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1158<F: Float>(t35151: F, t376: F, t89: F, t1053: F, t139173: F, t26590: F, t5968: F, t106551: F, t12277: F, t140237: F, t140239: F, t140241: F, t140253: F, t140263: F, t140268: F, t140274: F, t143: F, t144: F, t148686: F, t148692: F, t148703: F, t160: F, t1901: F, t26836: F, t26883: F, t28: F, t33060: F, t34947: F, t35229: F, t3578: F, t446: F, t558: F, t574: F, t5935: F, t5943: F, t605: F, t7357: F) -> (F, F, F) {
    let t148715 = t89 * t376 * t35151;
    let t148722 = t139173 * t1053;
    let t148726 = t26590 * t5968;
    let t148730 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t106551 * t5943 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t5935 * t26883 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t5935 * t26836 + t140237 / F::cast_from(9.0_f64) + t140239 / F::cast_from(9.0_f64) + t89 * t28 * t143 * t148686 * t160 / F::cast_from(3.0_f64) + t148692 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t140241 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t12277 * t7357 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t3578 * t33060 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t144 * t148703 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t140253 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t140263 + t446 * t574 * t605 * t34947 * t558 / F::cast_from(3.0_f64) - t148715 / F::cast_from(9.0_f64) - t446 * t574 * t35229 * t558 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t140268 + t140274 - t446 * t144 * t148722 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t144 * t148726;
    (t148722, t148726, t148730)
}
