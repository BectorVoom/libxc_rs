//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1223/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1223(t95362: f64, t95446: f64, t95499: f64, t96626: f64, t2118: f64, t4153: f64, t116: f64, t26153: f64, t10259: f64, t117: f64, t13232: f64, t13240: f64, t13244: f64, t13247: f64, t1459: f64, t1461: f64, t2113: f64, t2115: f64, t2327: f64, t2371: f64, t26716: f64, t26730: f64, t26733: f64, t26734: f64, t26737: f64, t26740: f64, t28974: f64, t4158: f64, t4162: f64, t4165: f64, t572: f64, t573: f64, t670: f64, t7373: f64, t7547: f64, t7553: f64, t7554: f64, t7557: f64, t95405: f64, param_d: f64) -> (f64, f64, f64) {
    let t96628 = t95362 + t95446 + t95499 + t96626;
    let t96633 = t4153 * t2118;
    let t96640 = t116 * t26153;
    let t96682 = 9.0_f64 * t1459 * t26740 + 18.0_f64 * t572 * t28974 * t2371 + 18.0_f64 * t572 * t96640 * t670 + 18.0_f64 * t572 * t26733 * t2371 + 9.0_f64 * t4158 * t7557 + 9.0_f64 * t26716 * t1461 + 18.0_f64 * t1459 * t26730 + 36.0_f64 * t1459 * t26734 + 18.0_f64 * t1459 * t26737 + 6.0_f64 * t2113 * t13240 + 18.0_f64 * t7547 * t4162 + 3.0_f64 * t2113 * t13247 + 3.0_f64 * t13232 * t2115 + 18.0_f64 * t2113 * t13244 + 9.0_f64 * t7547 * t4165 + 18.0_f64 * t4158 * t7554 + 3.0_f64 * t572 * t117 * t95405 + 6.0_f64 * t572 * t7553 * t10259 + 18.0_f64 * t572 * t2327 * t7373 + param_d * t96628 * t573;
    (t96628, t96633, t96682)
}
