//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1499/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1499<F: Float>(t108710: F, t108714: F, t118749: F, t1310: F, t13426: F, t18245: F, t1843: F, t21658: F, t2198: F, t2199: F, t2201: F, t2322: F, t30143: F, t31403: F, t31451: F, t31452: F, t31653: F, t31663: F, t31677: F, t4248: F, t4254: F, t508: F, t5523: F, t651: F, t6765: F, t75439: F, t7732: F, t8307: F, t8320: F, t8327: F, t8411: F, t85360: F) -> F {
    let t118911 = -F::new(2.0) * t651 * t21658 * t2198 - F::new(4.0) * t7732 * t31403 - F::new(2.0) * t75439 * t2199 - F::new(2.0) * t85360 * t2199 - F::new(2.0) * t18245 * t8307 + F::new(4.0) * t13426 * t8411 - F::new(2.0) * t651 * t6765 * t8320 - F::new(2.0) * t2322 * t31677 - F::new(2.0) * t4254 * t31677 - F::new(2.0) * t651 * t1310 * t31653 - F::new(4.0) * t4248 * t31452 - F::new(2.0) * t108710 * t2199 - F::new(2.0) * t108714 * t2199 + F::new(2.0) * t5523 * t31663 - F::new(2.0) * t651 * t508 * t118749 + F::new(2.0) * t75439 * t2201 + F::new(2.0) * t85360 * t2201 + F::new(2.0) * t30143 * t8327 - F::new(4.0) * t4248 * t31403 - F::new(4.0) * t651 * t1843 * t31451;
    t118911
}
