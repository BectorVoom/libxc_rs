//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 900/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk900<F: Float>(t123: F, t4961: F, t2673: F, t3623: F, t14360: F, t4947: F, t2643: F, t4776: F, t3634: F, t4768: F, t10917: F, t1382: F, t19: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16960 = t123 * t4961;
    let t16961 = t2673 * t16960;
    let t16962 = t3623 * t16961;
    let t16965 = t14360 * t4947;
    let t16968 = t2643 * t4776;
    let t16969 = t3634 * t16968;
    let t16975 = t2643 * t4768;
    let t16976 = t10917 * t16975;
    let t16979 = t19 * t1382;
    (t16960, t16961, t16962, t16965, t16968, t16969, t16975, t16976, t16979)
}
