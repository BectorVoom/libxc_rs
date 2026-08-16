//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1402/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1402(t1172: f64, t14368: f64, t14852: f64, t15397: f64, t2494: f64, t320: f64, t3946: f64, t4062: f64, t4120: f64, t54778: f64, t54852: f64, t54854: f64, t54866: f64, t56008: f64, t56034: f64, t56046: f64, t56053: f64, t57803: f64, t57946: f64, t57951: f64, t57953: f64, t57974: f64, t57994: f64, t58011: f64, t58035: f64, t58065: f64, t58083: f64, t58110: f64, t58140: f64, t58172: f64, t58196: f64, t58224: f64, t58257: f64, t58280: f64, t58302: f64, t58327: f64, t58376: f64, t58410: f64, t58444: f64, t58465: f64, t58488: f64, t58516: f64, t58547: f64, t58580: f64, t58797: f64, t58818: f64, t58839: f64, t58869: f64, t58883: f64, t58902: f64, t58919: f64, t58940: f64, t58962: f64, t945: f64) -> f64 {
    let t58978 = -t54852 - t54854 + 6.0_f64 * t3946 * t14852 * t2494 + t54866 + 3.0_f64 * t57946 + 4.0_f64 * t4062 * t14368 * t56034 - 2.0_f64 * t57951 + 6.0_f64 * t57953 + t1172 * t320 * (t58547 + t58224 + t58818 + t58516 + t58110 + t58035 + t58962 + t58302 + t58883 + t57974 + t58065 + t58869 + t58410 + t58196 + t58919 + t58140 + t58376 + t58839 + t58280 + t58257 + t57994 + t58940 + t58465 + t58172 + t58580 + t58444 + t58083 + t58011 + t58797 + t58902 + t58327 + t58488) * t945 + 12.0_f64 * t54778 * t57803 + t56008 + 6.0_f64 * t56053 * t15397 - 3.0_f64 * t3946 * t4120 * t56046;
    t58978
}
