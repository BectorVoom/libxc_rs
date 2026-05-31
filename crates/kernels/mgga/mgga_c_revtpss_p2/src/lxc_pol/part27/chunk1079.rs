//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1079/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1079<F: Float>(t13225: F, t3: F, t2327: F, t670: F, t116: F, t2371: F, t10259: F, t117: F, t1459: F, t1461: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, param_d: F) -> (F, F, F, F, F, F) {
    let t13226 = t3 * t13225;
    let t13232 = param_d * t13225;
    let t13240 = t2327 * t670;
    let t13243 = t116 * t670;
    let t13244 = t13243 * t2371;
    let t13247 = t117 * t10259;
    let t13250 = t13232 * t573 + F::cast_from(6.0_f64) * t13240 * t572 + F::cast_from(18.0_f64) * t13244 * t572 + F::cast_from(3.0_f64) * t13247 * t572 + F::cast_from(18.0_f64) * t1459 * t4162 + F::cast_from(9.0_f64) * t1459 * t4165 + F::cast_from(9.0_f64) * t1461 * t4158;
    (t13226, t13232, t13240, t13244, t13247, t13250)
}
