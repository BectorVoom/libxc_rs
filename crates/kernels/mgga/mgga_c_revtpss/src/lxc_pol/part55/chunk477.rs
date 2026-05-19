//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 477/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk477<F: Float>(t2985: F, t315: F, t2846: F, t2904: F, t963: F, t323: F, t300: F, t960: F, t988: F, t993: F, t378: F, t989: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2986 = F::new(1.0) / t2985;
    let t2987 = t315 * t2986;
    let t2994 = F::cast_from(0.40256666666666666667e0_f64) * t2846;
    let t3001 = F::new(0.137975e0) * t2904;
    let t3010 = t963 * t963;
    let t3011 = F::new(1.0) / t3010;
    let t3012 = t315 * t3011;
    let t3013 = t323 * t323;
    let t3014 = F::new(1.0) / t3013;
    let t3022 = t300 * t960;
    let t3037 = F::cast_from(0.11111111111111111111e-1_f64) * t2846;
    let t3046 = t988 * t993;
    let t3047 = t3046 * t378;
    let t3052 = t989 * t378;
    (t2986, t2987, t2994, t3001, t3011, t3012, t3014, t3022, t3037, t3046, t3047, t3052)
}
