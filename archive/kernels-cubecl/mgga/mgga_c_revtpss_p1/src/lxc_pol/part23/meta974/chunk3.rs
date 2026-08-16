//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3313/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3313<F: Float>(t1312: F, t13426: F, t1518: F, t18227: F, t18245: F, t21881: F, t22633: F, t2322: F, t27123: F, t28219: F, t30138: F, t4248: F, t4292: F, t5523: F, t5920: F, t670: F, t75439: F, t75931: F, t75941: F, t7889: F, t85308: F, t85329: F, t85360: F) -> F {
    let t86889 = F::cast_from(2.0_f64) * t1312 * t75931 + F::cast_from(6.0_f64) * t13426 * t5920 + F::cast_from(6.0_f64) * t1518 * t75439 + F::cast_from(6.0_f64) * t1518 * t85360 + F::cast_from(6.0_f64) * t18227 * t5920 + F::cast_from(6.0_f64) * t18245 * t4292 + F::cast_from(6.0_f64) * t21881 * t4248 + F::cast_from(6.0_f64) * t21881 * t7889 + F::cast_from(2.0_f64) * t22633 * t2322 + F::cast_from(2.0_f64) * t22633 * t5523 + F::cast_from(6.0_f64) * t27123 * t5920 + F::cast_from(6.0_f64) * t28219 * t5920 + F::cast_from(12.0_f64) * t30138 * t4292 + F::cast_from(2.0_f64) * t670 * t75941 + t85308 + F::cast_from(6.0_f64) * t85329;
    t86889
}
