//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1066/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1066<F: Float>(t20091: F, t4090: F, t14351: F, t4414: F, t1206: F, t353: F, t6161: F, t859: F, t14193: F, t19777: F, t14191: F, t892: F, t2416: F, t4110: F, t938: F, t14252: F, t8801: F) -> (F, F, F, F, F, F, F, F) {
    let t52159 = t20091 * t4090;
    let t52167 = t4414 * t14351;
    let t52179 = t859 * t353 * t1206 * t6161;
    let t52183 = t19777 * t14193;
    let t52188 = t859 * t892 * t14191;
    let t52191 = t2416 * t4110;
    let t52194 = t859 * t353 * t52191 * t938;
    let t52197 = t8801 * t14252;
    (t52159, t52167, t52179, t52183, t52188, t52191, t52194, t52197)
}
