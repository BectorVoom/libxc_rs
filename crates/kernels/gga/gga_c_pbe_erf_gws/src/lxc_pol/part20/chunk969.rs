//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 969/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk969<F: Float>(t10930: F, t1620: F, t2612: F, t2640: F, t2684: F, t7106: F, t5211: F, t3443: F, t572: F, t418: F, t1827: F, t587: F) -> (F, F, F, F) {
    let t10931 = t1620 * t10930;
    let t10932 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t10931;
    let t10933 = t2612 * t2640;
    let t10934 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t10933;
    let t10935 = t7106 * t2684;
    let t10937 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t5211 * t10935;
    let t10938 = t3443 * t572;
    let t10939 = t10938 * t418;
    let t10940 = t1827 * t10939;
    let t10942 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t587 * t10940;
    (t10932, t10934, t10937, t10942)
}
