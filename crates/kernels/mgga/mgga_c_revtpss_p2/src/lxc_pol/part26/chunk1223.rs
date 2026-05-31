//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1223/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1223<F: Float>(t95362: F, t95446: F, t95499: F, t96626: F, t2118: F, t4153: F, t116: F, t26153: F, t10259: F, t117: F, t13232: F, t13240: F, t13244: F, t13247: F, t1459: F, t1461: F, t2113: F, t2115: F, t2327: F, t2371: F, t26716: F, t26730: F, t26733: F, t26734: F, t26737: F, t26740: F, t28974: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, t670: F, t7373: F, t7547: F, t7553: F, t7554: F, t7557: F, t95405: F, param_d: F) -> (F, F, F) {
    let t96628 = t95362 + t95446 + t95499 + t96626;
    let t96633 = t4153 * t2118;
    let t96640 = t116 * t26153;
    let t96682 = F::cast_from(9.0_f64) * t1459 * t26740 + F::cast_from(18.0_f64) * t572 * t28974 * t2371 + F::cast_from(18.0_f64) * t572 * t96640 * t670 + F::cast_from(18.0_f64) * t572 * t26733 * t2371 + F::cast_from(9.0_f64) * t4158 * t7557 + F::cast_from(9.0_f64) * t26716 * t1461 + F::cast_from(18.0_f64) * t1459 * t26730 + F::cast_from(36.0_f64) * t1459 * t26734 + F::cast_from(18.0_f64) * t1459 * t26737 + F::cast_from(6.0_f64) * t2113 * t13240 + F::cast_from(18.0_f64) * t7547 * t4162 + F::cast_from(3.0_f64) * t2113 * t13247 + F::cast_from(3.0_f64) * t13232 * t2115 + F::cast_from(18.0_f64) * t2113 * t13244 + F::cast_from(9.0_f64) * t7547 * t4165 + F::cast_from(18.0_f64) * t4158 * t7554 + F::cast_from(3.0_f64) * t572 * t117 * t95405 + F::cast_from(6.0_f64) * t572 * t7553 * t10259 + F::cast_from(18.0_f64) * t572 * t2327 * t7373 + param_d * t96628 * t573;
    (t96628, t96633, t96682)
}
