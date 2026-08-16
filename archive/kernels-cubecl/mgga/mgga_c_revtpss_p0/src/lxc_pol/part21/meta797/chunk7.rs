//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2888/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2888<F: Float>(t1633: F, t3012: F, t11410: F, t11450: F, t11461: F, t11467: F, t11507: F, t11521: F, t15290: F, t311: F, t4673: F, t4711: F, t52207: F, t52209: F, t52211: F, t52213: F, t52216: F, t52218: F, t52221: F, t52223: F, t52226: F, t52229: F, t52405: F, t52426: F) -> F {
    let t52430 = t3012 * t1633;
    let t52433 = F::cast_from(0.6233709278045326953e3_f64) * t11507 * t4711 * t11467 + F::cast_from(0.11579025239058625248e4_f64) * t11450 * t4673 * t11410 + F::cast_from(0.10526802520742363173e2_f64) * t11461 * t15290 - F::cast_from(0.310907e-1_f64) * (t52405 + t52426) * t311 + F::cast_from(0.10526802520742363173e2_f64) * t52430 * t11521 + t52207 + t52209 - t52211 + t52213 - t52216 - t52218 - t52221 - t52223 - t52226 - t52229;
    t52433
}
