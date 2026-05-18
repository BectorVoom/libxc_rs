//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1000/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1000<F: Float>(t10919: F, t10921: F, t10923: F, t10926: F, t10929: F, t10932: F, t10934: F, t10937: F, t10942: F, t10944: F, t10946: F, t10950: F, t10952: F, t10959: F, t10963: F, t10967: F, t7784: F) -> F {
    let t11222 = -t10919 - t10921 - t10923 + t10926 + t10929 - t10932 + t10934 - t7784 - t10937 - t10942 + t10944 - t10946 + t10950 - t10952 - t10959 - t10963 - t10967;
    t11222
}
