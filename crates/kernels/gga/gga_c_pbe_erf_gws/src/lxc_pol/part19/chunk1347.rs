//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1347/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1347<F: Float>(t13953: F, t15345: F, t2409: F, t35654: F, t3959: F, t3909: F, t3955: F, t13796: F, t13859: F, t3896: F, t875: F, t1118: F, t3166: F) -> (F, F, F, F, F) {
    let t57702 = t13953 * t15345;
    let t57705 = t3959 * t2409 * t35654;
    let t57707 = t3955 * t3909;
    let t57711 = t13859 * t13796 * t3896 * t875;
    let t57719 = t13859 * t13796 * t1118 * t3166;
    (t57702, t57705, t57707, t57711, t57719)
}
