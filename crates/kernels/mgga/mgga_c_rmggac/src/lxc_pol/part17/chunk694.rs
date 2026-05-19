//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 694/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk694<F: Float>(t9901: F, t9925: F, t515: F, t235: F, t128: F, t1818: F, t118: F, t7418: F, t675: F, t1927: F, t1986: F, t1937: F) -> (F, F, F, F, F, F, F, F) {
    let t9926 = t9901 + t9925;
    let t9927 = t515 * t9926;
    let t9928 = t235 * t9927;
    let t9929 = F::cast_from(0.19957069503106347607e-1_f64) * t9928;
    let t9930 = t128 * t1818;
    let t9931 = t118 * t9930;
    let t9932 = t7418 * t9931;
    let t9933 = t675 * t9932;
    let t9934 = F::cast_from(0.85129199786595678796e-5_f64) * t9933;
    let t9935 = t1986 * t1927;
    let t9936 = t675 * t9935;
    let t9937 = F::cast_from(0.25538759935978703638e-4_f64) * t9936;
    let t9938 = t1986 * t1937;
    (t9926, t9927, t9929, t9932, t9934, t9935, t9937, t9938)
}
