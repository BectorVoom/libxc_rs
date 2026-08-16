//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1501/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1501(t10416: f64, t117575: f64, t1312: f64, t13435: f64, t13440: f64, t14310: f64, t18227: f64, t2178: f64, t2179: f64, t2322: f64, t27123: f64, t27126: f64, t31013: f64, t31016: f64, t31293: f64, t31299: f64, t31320: f64, t4248: f64, t5517: f64, t5523: f64, t569: f64, t5787: f64, t651: f64, t75485: f64, t7732: f64, t8254: f64, t8273: f64, t8274: f64, t8353: f64, t8367: f64) -> f64 {
    let t117711 = 2.0_f64 * t117575 * t1312 * t569 + 2.0_f64 * t1312 * t14310 * t2178 + 4.0_f64 * t1312 * t5787 * t8273 - 4.0_f64 * t5517 * t651 * t8273 - 2.0_f64 * t10416 * t8353 + 2.0_f64 * t10416 * t8367 - 4.0_f64 * t13435 * t8353 + 4.0_f64 * t13435 * t8367 + 2.0_f64 * t13440 * t8367 - 4.0_f64 * t18227 * t8254 - 2.0_f64 * t2179 * t75485 + 4.0_f64 * t2322 * t31293 - 4.0_f64 * t2322 * t31299 - 4.0_f64 * t2322 * t31320 - 4.0_f64 * t27123 * t8274 - 4.0_f64 * t27126 * t8274 - 2.0_f64 * t31013 * t4248 - 2.0_f64 * t31013 * t7732 - 4.0_f64 * t31016 * t7732 + 4.0_f64 * t31293 * t5523;
    t117711
}
