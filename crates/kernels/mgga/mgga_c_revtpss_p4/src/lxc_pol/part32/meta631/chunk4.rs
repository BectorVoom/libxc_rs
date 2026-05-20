//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2047/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2047<F: Float>(t108710: F, t108714: F, t109100: F, t13426: F, t18227: F, t18242: F, t1843: F, t2014: F, t2056: F, t2107: F, t25082: F, t26399: F, t27123: F, t27126: F, t28286: F, t28658: F, t28683: F, t28704: F, t28711: F, t29508: F, t30218: F, t30511: F, t30586: F, t4248: F, t5921: F, t651: F, t670: F, t7235: F, t73407: F, t7359: F, t7367: F, t7732: F, t7984: F) -> F {
    let t111174 = -F::new(4.0) * t4248 * t28711 - F::new(2.0) * t108710 * t2056 - F::new(2.0) * t108714 * t2056 - F::new(2.0) * t29508 * t7367 - F::new(4.0) * t651 * t1843 * t28683 - F::new(4.0) * t13426 * t7984 - F::new(4.0) * t18227 * t7984 - F::new(4.0) * t4248 * t28704 - F::new(4.0) * t27123 * t7984 - F::new(4.0) * t27126 * t7984 - F::new(4.0) * t7732 * t28704 - F::new(2.0) * t7235 * t30218 + F::new(6.0) * t25082 * t28286 * t109100 - F::new(2.0) * t26399 * t5921 - F::new(2.0) * t28658 * t5921 - F::new(2.0) * t7359 * t18242 - F::new(2.0) * t651 * t30511 * t670 - t2014 * t2107 * t73407 + F::new(6.0) * t7235 * t30586;
    t111174
}
