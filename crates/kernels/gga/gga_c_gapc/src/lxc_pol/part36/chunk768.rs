//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 768/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk768<F: Float>(t3231: F, t3243: F, t2786: F, t825: F, t996: F, t3218: F, t1560: F, t315: F, t2160: F, t2165: F, t3244: F, t126: F, t2190: F, t284: F, t3201: F, t763: F) -> (F, F, F, F, F, F) {
    let t10156 = t3243 * t3231;
    let t10158 = t2786 * t825;
    let t10159 = t996 * t10158;
    let t10160 = t10159 * t3218;
    let t10162 = t1560 * t315;
    let t10163 = t2160 * t10162;
    let t10165 = t2165 * t3244;
    let t10167 = t126 * t2190;
    let t10168 = t284 * t10167;
    let t10172 = t763 * t3201;
    (t10156, t10160, t10163, t10165, t10168, t10172)
}
