//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1369/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1369(t15528: f64, t9270: f64, t15445: f64, t15545: f64, t4414: f64, t15550: f64, t1161: f64, t1206: f64, t12213: f64, t14193: f64, t14922: f64, t14943: f64, t15081: f64, t15423: f64, t2408: f64, t2409: f64, t3066: f64, t3067: f64, t35000: f64, t35003: f64, t353: f64, t35889: f64, t3886: f64, t4088: f64, t4097: f64, t4110: f64, t43526: f64, t56753: f64, t56757: f64, t56761: f64, t6781: f64, t745: f64, t8589: f64, t859: f64) -> f64 {
    let t58333 = t9270 * t15528;
    let t58359 = t9270 * t15445;
    let t58361 = t4414 * t15545;
    let t58363 = t9270 * t15550;
    let t58376 = t3066 * t2409 * t3067 * t4110 * t3886 / 48.0_f64 - 7.0_f64 / 144.0_f64 * t58333 + t2408 * t2409 * t35889 * t4088 / 48.0_f64 + t2408 * t2409 * t6781 * t15423 / 24.0_f64 + t56753 / 384.0_f64 + t3066 * t2409 * t3067 * t15081 * t1161 / 24.0_f64 + t56757 / 384.0_f64 + t3066 * t2409 * t12213 * t14943 / 24.0_f64 - t56761 / 1536.0_f64 + t2408 * t2409 * t8589 * t14922 / 24.0_f64 - 7.0_f64 / 72.0_f64 * t58359 - 7.0_f64 / 144.0_f64 * t58361 - 7.0_f64 / 72.0_f64 * t58363 + t3066 * t2409 * t43526 * t4097 / 48.0_f64 + t35000 * t14193 / 48.0_f64 - t35003 * t859 * t353 * t1206 * t745 / 48.0_f64;
    t58376
}
