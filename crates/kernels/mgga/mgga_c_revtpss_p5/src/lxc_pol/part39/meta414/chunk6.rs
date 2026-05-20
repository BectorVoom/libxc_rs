//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1498/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1498<F: Float>(t114: F, t117477: F, t117517: F, t117560: F, t117572: F, t101522: F, t1312: F, t13435: F, t18153: F, t18163: F, t1911: F, t2178: F, t2181: F, t27123: F, t28219: F, t31066: F, t31067: F, t31070: F, t31084: F, t31309: F, t31318: F, t31324: F, t4151: F, t4248: F, t4254: F, t508: F, t5523: F, t651: F, t7889: F, t8278: F, t8280: F, t8362: F, t8363: F, t8369: F, t98484: F, t98487: F) -> (F, F) {
    let t115 = F::new(1.0) < t114;
    let t117575 = piecewise3::<F>(t115, F::new(0.0), t117477 + t117517 + t117560 + t117572);
    let t117579 = F::new(2.0) * t1312 * t31066 * t1911 + F::new(4.0) * t28219 * t8280 + F::new(2.0) * t7889 * t31084 + F::new(2.0) * t98484 * t2181 + F::new(4.0) * t98487 * t2181 + F::new(4.0) * t27123 * t8278 - F::new(2.0) * t18163 * t8363 - F::new(4.0) * t4254 * t31318 + F::new(4.0) * t13435 * t8369 + F::new(4.0) * t5523 * t31324 + F::new(2.0) * t101522 * t2181 + F::new(4.0) * t28219 * t8278 + F::new(2.0) * t4248 * t31084 + F::new(2.0) * t4248 * t31067 + F::new(2.0) * t1312 * t8362 * t4151 + F::new(2.0) * t7889 * t31067 + F::new(4.0) * t7889 * t31070 - F::new(2.0) * t651 * t18153 * t2178 + F::new(4.0) * t5523 * t31309 - F::new(2.0) * t651 * t508 * t117575;
    (t117575, t117579)
}
