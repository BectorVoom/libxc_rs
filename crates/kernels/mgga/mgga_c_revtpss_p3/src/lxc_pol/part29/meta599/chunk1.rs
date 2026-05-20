//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2040/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2040<F: Float>(t102714: F, t10416: F, t13435: F, t1519: F, t18153: F, t18163: F, t1911: F, t2014: F, t2055: F, t2106: F, t2322: F, t2371: F, t25082: F, t26377: F, t26383: F, t26392: F, t26399: F, t26405: F, t26699: F, t27153: F, t28167: F, t28704: F, t28750: F, t28760: F, t33183: F, t3829: F, t4254: F, t4257: F, t651: F, t7898: F, t7900: F, t7978: F, t7984: F, t7988: F, t8065: F, t95357: F, t98519: F) -> F {
    let t103917 = -F::new(2.0) * t18163 * t7984 - F::new(4.0) * t4254 * t28704 - F::new(6.0) * t28167 * t26405 * t98519 - F::new(6.0) * t25082 * t33183 * t27153 + t7898 * t26377 + F::new(6.0) * t2014 * t3829 * t2106 * t7900 + F::new(3.0) * t7898 * t26383 + t26699 * t1911 - F::new(2.0) * t651 * t18153 * t2055 - F::new(2.0) * t651 * t8065 * t2371 - F::new(2.0) * t95357 * t1519 - F::new(4.0) * t102714 * t1519 - F::new(4.0) * t26399 * t4257 - F::new(2.0) * t18163 * t7978 - F::new(4.0) * t4254 * t28760 - F::new(2.0) * t10416 * t7988 - F::new(4.0) * t13435 * t7988 - F::new(4.0) * t2322 * t28750 - t7898 * t26392;
    t103917
}
