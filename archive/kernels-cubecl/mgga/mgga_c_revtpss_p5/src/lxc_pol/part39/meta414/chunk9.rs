//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1501/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1501<F: Float>(t10416: F, t117575: F, t1312: F, t13435: F, t13440: F, t14310: F, t18227: F, t2178: F, t2179: F, t2322: F, t27123: F, t27126: F, t31013: F, t31016: F, t31293: F, t31299: F, t31320: F, t4248: F, t5517: F, t5523: F, t569: F, t5787: F, t651: F, t75485: F, t7732: F, t8254: F, t8273: F, t8274: F, t8353: F, t8367: F) -> F {
    let t117711 = F::cast_from(2.0_f64) * t117575 * t1312 * t569 + F::cast_from(2.0_f64) * t1312 * t14310 * t2178 + F::cast_from(4.0_f64) * t1312 * t5787 * t8273 - F::cast_from(4.0_f64) * t5517 * t651 * t8273 - F::cast_from(2.0_f64) * t10416 * t8353 + F::cast_from(2.0_f64) * t10416 * t8367 - F::cast_from(4.0_f64) * t13435 * t8353 + F::cast_from(4.0_f64) * t13435 * t8367 + F::cast_from(2.0_f64) * t13440 * t8367 - F::cast_from(4.0_f64) * t18227 * t8254 - F::cast_from(2.0_f64) * t2179 * t75485 + F::cast_from(4.0_f64) * t2322 * t31293 - F::cast_from(4.0_f64) * t2322 * t31299 - F::cast_from(4.0_f64) * t2322 * t31320 - F::cast_from(4.0_f64) * t27123 * t8274 - F::cast_from(4.0_f64) * t27126 * t8274 - F::cast_from(2.0_f64) * t31013 * t4248 - F::cast_from(2.0_f64) * t31013 * t7732 - F::cast_from(4.0_f64) * t31016 * t7732 + F::cast_from(4.0_f64) * t31293 * t5523;
    t117711
}
