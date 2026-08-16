//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 674/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk674<F: Float>(t1045: F, t1672: F, t211: F, t219: F, t5400: F, t5480: F, t1663: F, t995: F, t1023: F, t616: F, t996: F, t561: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7844 = t1672 * t1045;
    let t7845 = t211 * t7844;
    let t7853 = t5400 * t219;
    let t7877 = t5480 * t219;
    let t7899 = t995 * t1663;
    let t7945 = t1672 * t1023;
    let t7946 = t616 * t7945;
    let t7956 = t1672 * t996;
    let t7957 = t561 * t7956;
    (t7844, t7845, t7853, t7877, t7899, t7945, t7946, t7956, t7957)
}
