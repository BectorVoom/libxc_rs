//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1951/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1951<F: Float>(t1923: F, t26204: F, t7719: F, t101214: F, t2047: F, t101218: F, t101237: F, t101240: F, t101243: F, t101303: F, t101376: F, t2048: F, t25117: F, t25162: F, t26182: F, t28154: F, t28628: F, t28635: F, t6954: F, t7964: F, t92588: F, t95303: F) -> F {
    let t101929 = t1923 * t26204 * t7719;
    let t101935 = t2047 * t101214;
    let t101938 = t2047 * t101218;
    let t101949 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6954 * t28635 + t1923 * t2047 * t101303 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t101376 * t2048 + F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t101929 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t25117 * t7964 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t92588 * t28628 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t25162 * t101935 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t25162 * t101938 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t101237 * t26182 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t101240 * t26182 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t101243 * t26182 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t28154 * t95303;
    t101949
}
