//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1495/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1495<F: Float>(t10416: F, t13426: F, t13435: F, t18153: F, t18163: F, t18227: F, t1843: F, t2198: F, t2199: F, t2322: F, t27123: F, t27126: F, t31157: F, t31172: F, t31390: F, t31407: F, t3813: F, t4254: F, t651: F, t7732: F, t8307: F, t8321: F, t8327: F, t8393: F, t8406: F, t8407: F, t8411: F, t98535: F) -> F {
    let t117889 = -F::new(2.0) * t18153 * t2198 * t651 - F::new(2.0) * t1843 * t31157 * t651 - F::new(2.0) * t3813 * t651 * t8406 - F::new(2.0) * t10416 * t8393 - F::new(2.0) * t10416 * t8407 + F::new(2.0) * t10416 * t8411 - F::new(4.0) * t13426 * t8321 + F::new(4.0) * t13426 * t8327 - F::new(4.0) * t13435 * t8393 - F::new(4.0) * t13435 * t8407 + F::new(4.0) * t13435 * t8411 - F::new(2.0) * t18163 * t8407 - F::new(4.0) * t18227 * t8321 - F::new(2.0) * t2199 * t98535 - F::new(4.0) * t2322 * t31390 - F::new(4.0) * t2322 * t31407 + F::new(4.0) * t27123 * t8327 - F::new(4.0) * t27126 * t8307 - F::new(2.0) * t31172 * t7732 - F::new(4.0) * t31407 * t4254;
    t117889
}
