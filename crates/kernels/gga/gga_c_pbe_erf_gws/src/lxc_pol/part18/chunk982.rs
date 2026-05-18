//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 982/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk982<F: Float>(t11102: F, t173: F, t184: F, t199: F, t11039: F, t11040: F, t11044: F, t11047: F, t11050: F, t11053: F, t11058: F, t11062: F, t11063: F, t11064: F, t11066: F, t11086: F, t7915: F, t7919: F, t7927: F, t7934: F) -> (F, F) {
    let t11103 = t173 * t11102;
    let t11104 = t11103 * t184;
    let t11106 = F::new(2.0) / F::new(15.0) * t11104 * t199;
    let t11107 = t11039 - t7915 + t7919 + t7927 + t7934 + t11040 - t11044 - t11047 + t11050 - t11053 + t11058 + t11062 - t11063 - t11064 + t11066 + t11086 + t11106;
    (t11106, t11107)
}
