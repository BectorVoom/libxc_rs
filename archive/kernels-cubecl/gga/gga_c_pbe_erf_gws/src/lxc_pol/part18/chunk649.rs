//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 649/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk649<F: Float>(t125: F, t1463: F, t1467: F, t1471: F, t1475: F, t1482: F, t1486: F, t1499: F, t169: F, t2937: F, t2939: F, t299: F, t301: F, t3373: F, t3574: F) -> F {
    let t3577 = -t1463 + t1467 + t1471 - t1475 - t1482 + t1486 - t1499 - F::cast_from(0.23948468020509218188e-1_f64) * t2937 + F::cast_from(0.20267214298646782767e-1_f64) * t169 * t299 * t3373 * t301 + t3574 * t125 + F::cast_from(0.39914113367515363646e-1_f64) * t2939;
    t3577
}
