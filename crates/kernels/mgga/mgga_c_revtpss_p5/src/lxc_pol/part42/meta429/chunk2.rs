//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1498/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1498<F: Float>(t108710: F, t109150: F, t109153: F, t109242: F, t1312: F, t13426: F, t18227: F, t2198: F, t2201: F, t22506: F, t2322: F, t27123: F, t27126: F, t29508: F, t30138: F, t31390: F, t31401: F, t31456: F, t31459: F, t31674: F, t4248: F, t4254: F, t7732: F, t7889: F, t8307: F, t8321: F, t8327: F, t8393: F, t8411: F, t8413: F) -> F {
    let t118864 = F::new(2.0) * t1312 * t2198 * t22506 + F::new(2.0) * t108710 * t2201 + F::new(4.0) * t109150 * t2201 + F::new(4.0) * t109153 * t2201 + F::new(2.0) * t109242 * t2201 + F::new(4.0) * t13426 * t8413 + F::new(4.0) * t18227 * t8411 + F::new(4.0) * t18227 * t8413 - F::new(2.0) * t2322 * t31674 - F::new(4.0) * t27123 * t8393 - F::new(4.0) * t27126 * t8393 - F::new(2.0) * t29508 * t8307 - F::new(2.0) * t29508 * t8321 + F::new(4.0) * t30138 * t8327 - F::new(4.0) * t31390 * t7732 + F::new(4.0) * t31401 * t7889 + F::new(4.0) * t31456 * t4248 + F::new(4.0) * t31456 * t7889 + F::new(4.0) * t31459 * t4248 - F::new(2.0) * t31674 * t4254;
    t118864
}
