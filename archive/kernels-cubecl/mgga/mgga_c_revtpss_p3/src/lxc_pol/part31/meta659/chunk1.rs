//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2231/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2231<F: Float>(t108986: F, t1926: F, t2247: F, t5826: F, t60673: F, t6957: F, t101222: F, t101230: F, t101333: F, t10309: F, t108966: F, t108971: F, t108975: F, t108979: F, t108983: F, t25157: F, t25162: F, t25164: F, t28147: F, t28151: F, t28154: F, t34176: F, t6960: F) -> F {
    let t108987 = t1926 * t108986;
    let t108990 = t2247 * t5826;
    let t108995 = t60673 * t6957;
    let t109001 = -F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t101230 * t28151 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t108966 * t25164 - F::cast_from(10.0_f64) * t101333 * t28147 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t25162 * t108971 - F::cast_from(10.0_f64) * t25157 * t108975 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t25162 * t108979 - F::cast_from(5.0_f64) * t25157 * t108983 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t25162 * t108987 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t108990 * t25164 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t101222 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t108995 * t6960 + F::cast_from(20.0_f64) * t10309 * t34176 * t28147;
    t109001
}
