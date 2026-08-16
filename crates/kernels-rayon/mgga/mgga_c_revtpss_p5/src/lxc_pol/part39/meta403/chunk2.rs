//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1477/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1477(t31066: f64, t569: f64, t1453: f64, t8273: f64, t508: f64, t2178: f64, t4151: f64, t10416: f64, t1312: f64, t13435: f64, t13440: f64, t18163: f64, t2179: f64, t2181: f64, t2322: f64, t31013: f64, t31016: f64, t4254: f64, t5523: f64, t651: f64, t8254: f64, t8274: f64, t8278: f64, t8280: f64) -> (f64, f64, f64, f64, f64) {
    let t31067 = t31066 * t569;
    let t31070 = t8273 * t1453;
    let t31073 = t508 * t31066;
    let t31084 = t2178 * t4151;
    let t31087 = -2.0_f64 * t10416 * t2179 + 2.0_f64 * t10416 * t2181 + 2.0_f64 * t1312 * t31067 + 4.0_f64 * t1312 * t31070 + 2.0_f64 * t1312 * t31084 - 4.0_f64 * t13435 * t2179 + 4.0_f64 * t13435 * t2181 + 2.0_f64 * t13440 * t2181 - 2.0_f64 * t18163 * t2179 - 4.0_f64 * t2322 * t8254 - 4.0_f64 * t2322 * t8274 + 4.0_f64 * t2322 * t8278 + 4.0_f64 * t2322 * t8280 - 2.0_f64 * t31013 * t651 - 4.0_f64 * t31016 * t651 - 2.0_f64 * t31073 * t651 - 4.0_f64 * t4254 * t8254 - 4.0_f64 * t4254 * t8274 + 4.0_f64 * t5523 * t8278 + 4.0_f64 * t5523 * t8280;
    (t31067, t31070, t31073, t31084, t31087)
}
