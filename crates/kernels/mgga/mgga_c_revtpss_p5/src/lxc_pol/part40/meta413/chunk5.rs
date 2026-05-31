//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1499/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1499<F: Float>(t114: F, t117971: F, t118017: F, t101522: F, t1312: F, t13426: F, t18227: F, t2199: F, t2201: F, t2322: F, t27123: F, t27126: F, t28219: F, t31164: F, t31201: F, t31401: F, t31459: F, t4151: F, t4248: F, t49686: F, t508: F, t651: F, t75485: F, t75667: F, t7732: F, t7889: F, t8307: F, t8321: F, t8325: F, t8327: F, t8406: F, t98484: F, t98487: F) -> (F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t118019 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t117971 + t118017);
    let t118039 = -F::cast_from(2.0_f64) * t118019 * t508 * t651 + F::cast_from(2.0_f64) * t1312 * t4151 * t8406 + F::cast_from(2.0_f64) * t101522 * t2201 + F::cast_from(4.0_f64) * t13426 * t8325 - F::cast_from(4.0_f64) * t18227 * t8307 - F::cast_from(2.0_f64) * t2199 * t75485 + F::cast_from(2.0_f64) * t2201 * t49686 + F::cast_from(4.0_f64) * t2201 * t75667 + F::cast_from(2.0_f64) * t2201 * t98484 + F::cast_from(4.0_f64) * t2201 * t98487 + F::cast_from(4.0_f64) * t2322 * t31401 + F::cast_from(4.0_f64) * t2322 * t31459 - F::cast_from(4.0_f64) * t27123 * t8321 + F::cast_from(4.0_f64) * t27123 * t8325 - F::cast_from(4.0_f64) * t27126 * t8321 + F::cast_from(4.0_f64) * t28219 * t8325 + F::cast_from(4.0_f64) * t28219 * t8327 + F::cast_from(2.0_f64) * t31164 * t7889 - F::cast_from(4.0_f64) * t31201 * t4248 - F::cast_from(4.0_f64) * t31201 * t7732;
    (t118019, t118039)
}
