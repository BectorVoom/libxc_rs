//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1595/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1595<F: Float>(t43994: F, t44007: F, t448: F, t300: F, t1126: F, t12226: F, t12231: F, t3382: F, t3431: F, t408: F, t3385: F, t12230: F) -> (F, F, F, F, F) {
    let t44009 = (t43994 + t44007) * t448;
    let t44011 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t44009;
    let t44012 = t1126 * t12226;
    let t44014 = F::cast_from(0.2069040516770936012e4_f64) * t44012 * t12231;
    let t44017 = t408 / t3431 / t3382;
    let t44018 = t3385 * t3385;
    let t44021 = F::cast_from(0.62071215503128080361e4_f64) * t44017 * t44018 * t12230;
    (t44009, t44011, t44014, t44018, t44021)
}
