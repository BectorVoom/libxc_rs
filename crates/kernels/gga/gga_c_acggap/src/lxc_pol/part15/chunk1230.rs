//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1230/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1230<F: Float>(t34843: F, t34844: F, t34846: F, t34847: F, t34848: F, t34849: F, t37267: F, t37268: F, t37276: F, t37277: F, t37278: F, t39525: F, t39527: F, t39534: F, t39537: F, t39540: F, t39545: F, t39547: F) -> F {
    let t41693 = -t39525 / F::new(8.0) - t37267 + t37268 - F::new(7.0) / F::new(144.0) * t39527 + t34843 + F::new(0.68598428988911579156e-2) * t34844 + t34846 - t34847 + t34848 - F::new(0.2264262644851498949e-1) * t34849 - F::new(0.42874018118069736972e-3) * t39534 - F::new(0.42874018118069736972e-3) * t39537 - F::new(0.42874018118069736972e-3) * t39540 + t37276 - t37277 + t37278 - F::new(0.28582678745379824648e-3) * t39545 - F::new(0.21437009059034868486e-3) * t39547;
    t41693
}
