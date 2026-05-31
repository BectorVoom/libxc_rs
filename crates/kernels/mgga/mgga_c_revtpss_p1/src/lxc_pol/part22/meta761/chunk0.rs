//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2842/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2842<F: Float>(t2857: F, t3154: F, t2251: F, t11262: F, t3127: F, t3129: F, t11988: F, t3106: F, t271: F, t2852: F, t1054: F, t11970: F) -> (F, F, F, F, F, F) {
    let t43174 = t3154 * t2857;
    let t43175 = t43174 * t2251;
    let t43204 = t3127 * t11262 * t3129;
    let t43215 = t3106 * t11988;
    let t43222 = F::cast_from(1.0_f64) / t271 / t2852;
    let t43238 = t1054 * t11970;
    (t43174, t43175, t43204, t43215, t43222, t43238)
}
