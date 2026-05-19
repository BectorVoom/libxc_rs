//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1134/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1134<F: Float>(t35573: F, t31363: F, t31374: F, t31377: F, t31381: F, t31390: F, t31392: F, t31407: F, t35545: F, t35550: F, t35553: F, t35557: F, t35560: F, t35562: F, t35563: F, t35564: F, t35567: F, t35570: F) -> F {
    let t35574 = F::cast_from(0.31448092289604152068e-2_f64) * t35573;
    let t35575 = F::cast_from(0.17149607247227894789e-2_f64) * t35545 - t35550 + t35553 - t35557 - F::cast_from(0.15724046144802076034e-2_f64) * t31363 + F::new(0.16809375e0) * t31374 + F::new(35.0) / F::new(432.0) * t35560 - t31377 - t31381 + t35562 + t35563 + t31390 - t31392 - t31407 + F::cast_from(0.13719685797782315831e-1_f64) * t35564 + F::cast_from(0.21437009059034868486e-3_f64) * t35567 + t35570 - t35574;
    t35575
}
