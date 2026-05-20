//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1497/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1497<F: Float>(t114: F, t118655: F, t118688: F, t118728: F, t118746: F, t1312: F, t13426: F, t1453: F, t18227: F, t18245: F, t2322: F, t27123: F, t27126: F, t28219: F, t30143: F, t31382: F, t31407: F, t31459: F, t31653: F, t31654: F, t31660: F, t4248: F, t4254: F, t5517: F, t5523: F, t569: F, t651: F, t7732: F, t7889: F, t8325: F, t8327: F, t8406: F, t8407: F, t8411: F, t8413: F) -> (F, F) {
    let t115 = F::new(1.0) < t114;
    let t118749 = piecewise3::<F>(t115, F::new(0.0), t118655 + t118688 + t118728 + t118746);
    let t118822 = F::new(2.0) * t118749 * t1312 * t569 + F::new(2.0) * t1312 * t1453 * t31653 - F::new(4.0) * t5517 * t651 * t8406 - F::new(4.0) * t13426 * t8407 - F::new(4.0) * t18227 * t8407 + F::new(2.0) * t18245 * t8327 + F::new(2.0) * t2322 * t31654 - F::new(4.0) * t2322 * t31660 - F::new(4.0) * t27123 * t8407 + F::new(4.0) * t27123 * t8411 - F::new(4.0) * t27126 * t8407 + F::new(4.0) * t28219 * t8411 + F::new(4.0) * t28219 * t8413 + F::new(2.0) * t30143 * t8325 + F::new(4.0) * t31382 * t4248 - F::new(4.0) * t31407 * t4248 - F::new(4.0) * t31407 * t7732 + F::new(4.0) * t31459 * t7889 + F::new(2.0) * t31654 * t5523 - F::new(4.0) * t31660 * t4254;
    (t118749, t118822)
}
