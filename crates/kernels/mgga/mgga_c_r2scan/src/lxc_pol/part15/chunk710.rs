//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 710/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk710<F: Float>(t1917: F, t1938: F, t1966: F, t1990: F, t2000: F, t2030: F, t208: F, t226: F, t390: F, t5331: F, t5335: F, t5384: F, t5392: F, t5486: F, t5490: F, t5504: F, t5508: F, t625: F, t668: F, t682: F, t686: F, t699: F, t713: F) -> F {
    let t5511 = F::cast_from(0.21687162600603479684e-1_f64) * t625 * t1990 * t713 + F::cast_from(0.68493333333333333332e-1_f64) * t625 * t2000 * t682 - F::cast_from(0.16867793133802706421e-1_f64) * t625 * t5486 * t226 - F::cast_from(0.53272592592592592592e-1_f64) * t625 * t5490 * t208 - F::cast_from(0.51369999999999999999e-1_f64) * t625 * t668 * t2030 - F::cast_from(0.16265371950452609763e-1_f64) * t625 * t699 * t1917 - t5331 + t5335 + t5384 - t5392 + F::cast_from(0.19827150884348052633e2_f64) * t686 * t1966 * t1938 + F::cast_from(0.20547999999999999999e0_f64) * t390 * t5504 + F::cast_from(0.10274e0_f64) * t390 * t5508;
    t5511
}
