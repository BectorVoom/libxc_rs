//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 897/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk897<F: Float>(t16925: F, t16927: F, t16929: F, t16931: F, t16936: F, t16939: F, t16945: F, t16948: F, t16953: F, t16955: F, t16957: F, t5416: F, t723: F, t16959: F, t17047: F, t17049: F, t17051: F, t17053: F, t17055: F, t17058: F, t17063: F, t17067: F, t17069: F, t17071: F) -> (F, F) {
    let t18208 = t16925 + t16927 - t16929 - t16931 - t16936 + t16939 - t16945 - t16948 - t16953 + t16955 - t16957;
    let t18209 = t5416 * t723;
    let t18211 = -t16959 + t17047 + t17049 - t17051 + t17053 + t17055 - t17058 + 8.0 / 3.0 * t18209 - t17063 + t17067 - t17069 - t17071;
    (t18208, t18211)
}
