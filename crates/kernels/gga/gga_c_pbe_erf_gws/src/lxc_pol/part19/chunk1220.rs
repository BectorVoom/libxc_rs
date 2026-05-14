//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1220/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1220<F: Float>(t1172: F, t14368: F, t14852: F, t15397: F, t2494: F, t320: F, t3946: F, t4062: F, t4120: F, t54778: F, t54852: F, t54854: F, t54866: F, t56008: F, t56034: F, t56046: F, t56053: F, t57803: F, t57946: F, t57951: F, t57953: F, t57974: F, t57994: F, t58011: F, t58035: F, t58065: F, t58083: F, t58110: F, t58140: F, t58172: F, t58196: F, t58224: F, t58257: F, t58280: F, t58302: F, t58327: F, t58376: F, t58410: F, t58444: F, t58465: F, t58488: F, t58516: F, t58547: F, t58580: F, t58797: F, t58818: F, t58839: F, t58869: F, t58883: F, t58902: F, t58919: F, t58940: F, t58962: F, t945: F) -> (F,) {
    let t58978 = -t54852 - t54854 + 6.0 * t3946 * t14852 * t2494 + t54866 + 3.0 * t57946 + 4.0 * t4062 * t14368 * t56034 - 2.0 * t57951 + 6.0 * t57953 + t1172 * t320 * (t58547 + t58224 + t58818 + t58516 + t58110 + t58035 + t58962 + t58302 + t58883 + t57974 + t58065 + t58869 + t58410 + t58196 + t58919 + t58140 + t58376 + t58839 + t58280 + t58257 + t57994 + t58940 + t58465 + t58172 + t58580 + t58444 + t58083 + t58011 + t58797 + t58902 + t58327 + t58488) * t945 + 12.0 * t54778 * t57803 + t56008 + 6.0 * t56053 * t15397 - 3.0 * t3946 * t4120 * t56046;
    (t58978,)
}
