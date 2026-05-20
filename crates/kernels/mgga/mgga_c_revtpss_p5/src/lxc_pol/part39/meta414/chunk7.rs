//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1499/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1499<F: Float>(t10416: F, t1312: F, t13426: F, t13435: F, t1453: F, t18227: F, t1843: F, t2179: F, t2181: F, t2322: F, t27123: F, t27126: F, t31066: F, t31070: F, t31248: F, t31292: F, t31314: F, t31318: F, t4248: F, t4254: F, t49686: F, t5523: F, t651: F, t75485: F, t75667: F, t8254: F, t8278: F, t8280: F, t8363: F, t98535: F) -> F {
    let t117622 = F::new(4.0) * t1312 * t1453 * t31292 - F::new(2.0) * t1843 * t31066 * t651 - F::new(2.0) * t10416 * t8363 - F::new(4.0) * t13426 * t8254 + F::new(4.0) * t13426 * t8278 - F::new(4.0) * t13435 * t8363 + F::new(4.0) * t18227 * t8278 + F::new(4.0) * t18227 * t8280 - F::new(2.0) * t2179 * t49686 - F::new(4.0) * t2179 * t75667 - F::new(2.0) * t2179 * t98535 + F::new(2.0) * t2181 * t49686 + F::new(2.0) * t2181 * t75485 + F::new(4.0) * t2181 * t75667 - F::new(4.0) * t2322 * t31318 + F::new(4.0) * t27123 * t8280 - F::new(4.0) * t27126 * t8254 + F::new(4.0) * t31070 * t4248 + F::new(4.0) * t31248 * t5523 - F::new(4.0) * t31314 * t4254;
    t117622
}
