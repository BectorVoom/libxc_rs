//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 911/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk911<F: Float>(t2271: F, t3162: F, t372: F, t7048: F, t7050: F, t7095: F, t7097: F, t8644: F, t8646: F, t8647: F, t881: F, t9005: F, t9063: F, t9066: F, t9592: F) -> F {
    let t9804 = t2271 * t3162;
    let t9810 = t372 * t9005 - t7048 - t7050 + t8644 + t9592 + t7095 + t7097 - F::cast_from(0.2363e1_f64) * t9804 - F::cast_from(0.2363e1_f64) * t881 * t9063 - F::cast_from(0.2363e1_f64) * t881 * t9066 - t8646 + t8647;
    t9810
}
