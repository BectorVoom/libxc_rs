//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1500/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1500<F: Float>(t118019: F, t1312: F, t13426: F, t13435: F, t1453: F, t18163: F, t18227: F, t1911: F, t2199: F, t2201: F, t2322: F, t27123: F, t31157: F, t31158: F, t31161: F, t31169: F, t31172: F, t31382: F, t31390: F, t31451: F, t4248: F, t4254: F, t49686: F, t569: F, t75485: F, t75667: F, t7732: F, t7889: F, t8307: F, t8325: F, t8393: F, t8413: F, t98484: F, t98487: F) -> F {
    let t118083 = F::cast_from(2.0_f64) * t1312 * t31157 * t1911 + F::cast_from(4.0_f64) * t2322 * t31382 + F::cast_from(4.0_f64) * t1312 * t31451 * t1453 + F::cast_from(2.0_f64) * t1312 * t118019 * t569 + F::cast_from(4.0_f64) * t4248 * t31161 + F::cast_from(2.0_f64) * t7889 * t31158 - F::cast_from(2.0_f64) * t4248 * t31172 - F::cast_from(2.0_f64) * t98484 * t2199 - F::cast_from(4.0_f64) * t98487 * t2199 - F::cast_from(4.0_f64) * t27123 * t8307 + F::cast_from(4.0_f64) * t13435 * t8413 - F::cast_from(2.0_f64) * t18163 * t8393 - F::cast_from(4.0_f64) * t4254 * t31390 - F::cast_from(2.0_f64) * t7732 * t31169 + F::cast_from(2.0_f64) * t75485 * t2201 + F::cast_from(4.0_f64) * t18227 * t8325 - F::cast_from(2.0_f64) * t49686 * t2199 - F::cast_from(4.0_f64) * t75667 * t2199 - F::cast_from(4.0_f64) * t13426 * t8307 + F::cast_from(2.0_f64) * t4248 * t31158;
    t118083
}
