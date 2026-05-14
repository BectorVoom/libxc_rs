//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 963/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk963<F: Float>(t2145: F, t3916: F, t2150: F, t3759: F, t6416: F, t9182: F, t2319: F, t3749: F, t1109: F, t857: F, t858: F, t856: F, t8848: F, t1139: F, t2169: F, t3108: F) -> (F, F, F, F, F, F) {
    let t12054 = t3916 * t2145;
    let t12056 = t12054 * t2150 / 48.0;
    let t12057 = t6416 * t3759;
    let t12060 = 35.0 / 216.0 * t9182;
    let t12061 = t2319 * t3749;
    let t12064 = t857 * t858 * t1109;
    let t12065 = t856 * t12064;
    let t12067 = t8848 * t12065 / 96.0;
    let t12068 = t2169 * t1139;
    let t12069 = t856 * t12068;
    let t12071 = t3108 * t12069 / 24.0;
    (t12056, t12057, t12060, t12061, t12067, t12071)
}
