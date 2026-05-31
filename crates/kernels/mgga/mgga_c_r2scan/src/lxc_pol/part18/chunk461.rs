//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 461/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk461<F: Float>(t1267: F, t2104: F, t2279: F, t2281: F, t2289: F, t2293: F, t2295: F, t2302: F, t2304: F, t269: F, t550: F, t864: F, t870: F) -> F {
    let t2312 = F::cast_from(2.0_f64) * t2279 * t864 - F::cast_from(1.0_f64) * t2281 * t864 + F::cast_from(1.0_f64) * t2289 * t864 + F::cast_from(0.2845018947250181111e-1_f64) * t2293 * t2295 - F::cast_from(0.20235332025531322028e-2_f64) * t2302 * t2104 * t269 * t2304 + F::cast_from(0.52158680699586653702e-1_f64) * t870 * t550 * t1267;
    t2312
}
