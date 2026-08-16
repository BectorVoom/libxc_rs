//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1174/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1174(t1882: f64, t35035: f64, t11593: f64, t12714: f64, t140395: f64, t140397: f64, t140412: f64, t140424: f64, t140426: f64, t1557: f64, t1901: f64, t26955: f64, t26999: f64, t3052: f64, t3188: f64, t3281: f64, t33039: f64, t3450: f64, t446: f64, t569: f64, t574: f64, t5935: f64, t7357: f64, t7407: f64, t7414: f64, t9144: f64) -> f64 {
    let t149256 = t1882 * t35035;
    let t149263 = 2.0_f64 / 27.0_f64 * t1901 * t12714 * t7407 * t1557 * t3188 - 4.0_f64 / 9.0_f64 * t11593 * t9144 * t7357 * t3052 - 2.0_f64 / 9.0_f64 * t3281 * t569 * t7414 * t3052 + t140395 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t140397 - t140412 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t574 * t5935 * t26955 - 2.0_f64 / 9.0_f64 * t140424 - 4.0_f64 / 9.0_f64 * t149256 + 2.0_f64 / 27.0_f64 * t140426 - 2.0_f64 * t1901 * t26999 * t33039 * t3450;
    t149263
}
