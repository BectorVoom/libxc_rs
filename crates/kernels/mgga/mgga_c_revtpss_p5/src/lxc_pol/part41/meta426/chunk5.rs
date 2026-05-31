//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1491/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1491<F: Float>(t13426: F, t18227: F, t18245: F, t2179: F, t2181: F, t27123: F, t27126: F, t28219: F, t31248: F, t31299: F, t31309: F, t31318: F, t31324: F, t4248: F, t651: F, t6765: F, t75439: F, t7732: F, t7889: F, t8254: F, t8273: F, t8278: F, t8353: F, t8363: F, t8369: F, t85360: F) -> F {
    let t118456 = -F::cast_from(2.0_f64) * t651 * t6765 * t8273 - F::cast_from(4.0_f64) * t13426 * t8363 - F::cast_from(4.0_f64) * t18227 * t8363 - F::cast_from(2.0_f64) * t18245 * t8254 + F::cast_from(2.0_f64) * t18245 * t8278 - F::cast_from(2.0_f64) * t2179 * t75439 - F::cast_from(2.0_f64) * t2179 * t85360 + F::cast_from(2.0_f64) * t2181 * t85360 - F::cast_from(4.0_f64) * t27123 * t8353 - F::cast_from(4.0_f64) * t27123 * t8363 + F::cast_from(4.0_f64) * t27123 * t8369 - F::cast_from(4.0_f64) * t27126 * t8353 - F::cast_from(4.0_f64) * t27126 * t8363 + F::cast_from(4.0_f64) * t28219 * t8369 + F::cast_from(4.0_f64) * t31248 * t4248 - F::cast_from(4.0_f64) * t31299 * t7732 + F::cast_from(4.0_f64) * t31309 * t7889 - F::cast_from(4.0_f64) * t31318 * t4248 - F::cast_from(4.0_f64) * t31318 * t7732 + F::cast_from(4.0_f64) * t31324 * t4248;
    t118456
}
