//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 641/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk641<F: Float>(t1023: F, t1672: F, t616: F, t996: F, t561: F, t1076: F, t1365: F, t153: F, t1333: F, t960: F, t1438: F, t2515: F, t409: F, t1326: F, t959: F, t40: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7945 = t1672 * t1023;
    let t7946 = t616 * t7945;
    let t7956 = t1672 * t996;
    let t7957 = t561 * t7956;
    let t7981 = t153 * t1365 * t1076;
    let t7986 = t1333 * t960;
    let t7988 = t1438 * t960;
    let t7990 = t409 * t2515;
    let t7996 = t959 * t1326;
    let t7997 = t40 * t7996;
    (t7945, t7946, t7956, t7957, t7981, t7986, t7988, t7990, t7996, t7997)
}
