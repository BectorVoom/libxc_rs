//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3305/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3305<F: Float>(t1343: F, t13600: F, t1450: F, t1868: F, t198: F, t22466: F, t22486: F, t39419: F, t39422: F, t4139: F, t46297: F, t46963: F, t46970: F, t47753: F, t47760: F, t48157: F, t48159: F, t532: F, t5536: F, t5591: F, t6836: F, t75379: F, t85390: F, t85391: F, t85442: F, t85466: F, t85482: F, t85498: F, t85887: F, t85888: F, t85889: F, t86291: F, t86308: F, t86340: F, t86691: F, t86718: F) -> F {
    let t86728 = t47753 + t85390 - t85391 - t47760 + F::cast_from(18.0_f64) * t5536 * t13600 * t6836 + F::cast_from(18.0_f64) * t5536 * t75379 * t1868 + F::cast_from(18.0_f64) * t5536 * t22486 * t5591 - t46297 + F::cast_from(3.0_f64) * t198 * t1343 * t85442 + t198 * t532 * (t85466 + t85482 + t85498 + t86291 + t86308 + t86340 + t86691 + t86718) * t1450 - t39419 - t39422 + t85887 - F::cast_from(9.0_f64) * t4139 * t22466 * t5591 - t85888 - t85889 - t48157 + t48159 - t46963 + t46970;
    t86728
}
