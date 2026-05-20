//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2801/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2801<F: Float>(t51512: F, t10872: F, t40298: F, t40303: F, t40307: F, t40311: F, t40314: F, t40316: F, t40318: F, t51498: F, t51505: F, t51507: F, t820: F) -> F {
    let t51513 = F::cast_from(0.39029762157531132076e-1_f64) * t51512;
    let t51515 = -F::cast_from(0.39512695097613069591e1_f64) * t820 * t51498 * t10872 - F::cast_from(0.29272321618148349057e-1_f64) * t40298 - F::cast_from(0.16463622957338778996e-1_f64) * t51505 + F::cast_from(0.43902994552903410656e-1_f64) * t51507 - F::cast_from(0.21951497276451705329e-1_f64) * t40303 + F::cast_from(0.54878743191129263322e-2_f64) * t40307 - F::cast_from(0.54878743191129263322e-2_f64) * t40311 - t40314 + t40316 + t51513 + F::cast_from(0.33133632253434461091e-3_f64) * t40318;
    t51515
}
