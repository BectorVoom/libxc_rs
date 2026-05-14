//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1195/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1195<F: Float>(t15528: F, t9270: F, t15445: F, t15545: F, t4414: F, t15550: F, t1161: F, t1206: F, t12213: F, t14193: F, t14922: F, t14943: F, t15081: F, t15423: F, t2408: F, t2409: F, t3066: F, t3067: F, t35000: F, t35003: F, t353: F, t35889: F, t3886: F, t4088: F, t4097: F, t4110: F, t43526: F, t56753: F, t56757: F, t56761: F, t6781: F, t745: F, t8589: F, t859: F) -> (F,) {
    let t58333 = t9270 * t15528;
    let t58359 = t9270 * t15445;
    let t58361 = t4414 * t15545;
    let t58363 = t9270 * t15550;
    let t58376 = t3066 * t2409 * t3067 * t4110 * t3886 / 48.0 - 7.0 / 144.0 * t58333 + t2408 * t2409 * t35889 * t4088 / 48.0 + t2408 * t2409 * t6781 * t15423 / 24.0 + t56753 / 384.0 + t3066 * t2409 * t3067 * t15081 * t1161 / 24.0 + t56757 / 384.0 + t3066 * t2409 * t12213 * t14943 / 24.0 - t56761 / 1536.0 + t2408 * t2409 * t8589 * t14922 / 24.0 - 7.0 / 72.0 * t58359 - 7.0 / 144.0 * t58361 - 7.0 / 72.0 * t58363 + t3066 * t2409 * t43526 * t4097 / 48.0 + t35000 * t14193 / 48.0 - t35003 * t859 * t353 * t1206 * t745 / 48.0;
    (t58376,)
}
