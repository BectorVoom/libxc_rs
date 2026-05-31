//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1894/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1894<F: Float>(t26169: F, t7702: F, t28640: F, t6954: F, t1923: F, t28089: F, t7348: F, t26205: F, t26204: F, t7719: F, t101214: F, t2047: F) -> (F, F, F, F, F, F) {
    let t101901 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t7702 * t26169;
    let t101903 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t6954 * t28640;
    let t101906 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1923 * t7348 * t28089;
    let t101907 = t7702 * t26205;
    let t101929 = t1923 * t26204 * t7719;
    let t101935 = t2047 * t101214;
    (t101901, t101903, t101906, t101907, t101929, t101935)
}
