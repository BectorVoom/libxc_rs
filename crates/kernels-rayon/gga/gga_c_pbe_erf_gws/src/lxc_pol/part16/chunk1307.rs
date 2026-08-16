//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1307/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1307(t14843: f64, t804: f64, t15097: f64, t2053: f64, t15081: f64, t2376: f64, t829: f64, t830: f64, t52901: f64, t1115: f64, t4083: f64, t50904: f64, t52276: f64, t52889: f64, t52893: f64, t52904: f64, t52908: f64, t52910: f64, t52912: f64, t52917: f64, t52919: f64, t52924: f64, t52928: f64, t827: f64, t9265: f64) -> (f64, f64, f64) {
    let t54866 = 6.0_f64 * t804 * t14843;
    let t54867 = t15097 * t2053;
    let t54880 = t2376 * t15081;
    let t54882 = t829 * t830 * t54880;
    let t54886 = 7.0_f64 / 576.0_f64 * t52901;
    let t54895 = t52889 / 768.0_f64 + t52893 / 16.0_f64 - t9265 * t4083 / 96.0_f64 - t1115 * t52276 / 48.0_f64 - t827 * t54882 / 48.0_f64 - 7.0_f64 / 72.0_f64 * t50904 - t54886 + t52904 / 384.0_f64 - t52908 / 96.0_f64 - t52910 / 24.0_f64 - t52912 / 24.0_f64 + t52917 / 96.0_f64 + t52919 / 24.0_f64 - t52924 / 768.0_f64 - 5.0_f64 / 192.0_f64 * t52928;
    (t54866, t54867, t54895)
}
