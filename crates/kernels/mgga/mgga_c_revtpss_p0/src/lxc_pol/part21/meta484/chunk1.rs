//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2063/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2063<F: Float>(t15283: F, t953: F, t1622: F, t2944: F, t1634: F, t2988: F, t15127: F, t15168: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15163: F, t15166: F, t15170: F, t15173: F) -> (F, F, F, F, F) {
    let t15284 = t15283 * t953;
    let t15287 = t1622 * t2944;
    let t15290 = t1634 * t2988;
    let t15301 = F::cast_from(0.22954444444444444444e0_f64) * t15127;
    let t15312 = F::cast_from(0.27785333333333333334e0_f64) * t15168;
    let t15315 = -F::cast_from(0.34431666666666666667e0_f64) * t15137 - F::cast_from(0.57386111111111111112e0_f64) * t15142 + F::cast_from(0.20659e1_f64) * t15147 + F::cast_from(0.103295e1_f64) * t15151 + F::cast_from(0.20659e1_f64) * t15156 - F::cast_from(0.309885e1_f64) * t15160 + F::cast_from(0.20839e0_f64) * t15163 - F::cast_from(0.62517e0_f64) * t15166 - t15312 + F::cast_from(0.46308888888888888889e-1_f64) * t15170 - F::cast_from(0.69463333333333333334e-1_f64) * t15173;
    (t15284, t15287, t15290, t15301, t15315)
}
