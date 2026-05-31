//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1500/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1500<F: Float>(t10416: F, t1310: F, t13426: F, t13440: F, t18163: F, t18227: F, t2179: F, t2322: F, t27123: F, t31016: F, t31073: F, t31248: F, t31292: F, t31299: F, t31309: F, t31314: F, t31320: F, t31324: F, t3813: F, t4248: F, t4254: F, t651: F, t7732: F, t8254: F, t8274: F, t8280: F, t8353: F, t8362: F, t8369: F, t98484: F, t98487: F) -> F {
    let t117666 = -F::cast_from(4.0_f64) * t1310 * t31292 * t651 - F::cast_from(2.0_f64) * t3813 * t651 * t8362 + F::cast_from(2.0_f64) * t10416 * t8369 - F::cast_from(4.0_f64) * t13426 * t8274 + F::cast_from(4.0_f64) * t13426 * t8280 + F::cast_from(2.0_f64) * t13440 * t8369 - F::cast_from(2.0_f64) * t18163 * t8353 - F::cast_from(4.0_f64) * t18227 * t8274 - F::cast_from(2.0_f64) * t2179 * t98484 - F::cast_from(4.0_f64) * t2179 * t98487 + F::cast_from(4.0_f64) * t2322 * t31248 + F::cast_from(4.0_f64) * t2322 * t31309 - F::cast_from(4.0_f64) * t2322 * t31314 + F::cast_from(4.0_f64) * t2322 * t31324 - F::cast_from(4.0_f64) * t27123 * t8254 - F::cast_from(4.0_f64) * t31016 * t4248 - F::cast_from(2.0_f64) * t31073 * t4248 - F::cast_from(2.0_f64) * t31073 * t7732 - F::cast_from(4.0_f64) * t31299 * t4254 - F::cast_from(4.0_f64) * t31320 * t4254;
    t117666
}
