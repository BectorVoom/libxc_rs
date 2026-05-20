//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2432/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2432<F: Float>(t11452: F, t2962: F, t41306: F, t3335: F, t1071: F, t3043: F, t12032: F, t342: F, t11902: F, t378: F, t3046: F, t3259: F) -> (F, F, F, F, F, F, F, F) {
    let t41895 = t2962 * t11452;
    let t41908 = F::cast_from(0.17757530864197530864e0_f64) * t41306;
    let t41936 = t3335 * t3335;
    let t41937 = F::new(1.0) / t41936;
    let t41993 = t3043 * t1071;
    let t42013 = F::cast_from(0.86419753086419753087e-1_f64) * t41306;
    let t42038 = t342 * t12032;
    let t42041 = t11902 * t378;
    let t42044 = t3046 * t3259;
    (t41895, t41908, t41937, t41993, t42013, t42038, t42041, t42044)
}
