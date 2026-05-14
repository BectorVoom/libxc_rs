//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 877/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk877<F: Float>(t3522: F, t5480: F, t639: F, t1630: F, t3518: F, t3512: F, t5493: F, t1620: F, t2612: F, t2640: F, t2684: F, t7106: F, t5211: F, t3443: F, t572: F, t418: F) -> (F, F, F, F, F, F) {
    let t10924 = t5480 * t3522;
    let t10925 = t639 * t10924;
    let t10926 = 8.0 / 81.0 * t10925;
    let t10927 = t1630 * t3518;
    let t10928 = t639 * t10927;
    let t10929 = 8.0 / 135.0 * t10928;
    let t10930 = t5493 * t3512;
    let t10931 = t1620 * t10930;
    let t10932 = 16.0 / 45.0 * t10931;
    let t10933 = t2612 * t2640;
    let t10934 = 16.0 / 135.0 * t10933;
    let t10935 = t7106 * t2684;
    let t10937 = 16.0 / 45.0 * t5211 * t10935;
    let t10938 = t3443 * t572;
    let t10939 = t10938 * t418;
    (t10926, t10929, t10932, t10934, t10937, t10939)
}
