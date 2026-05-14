//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 599/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk599<F: Float>(t2083: F, t459: F, t1175: F, t5926: F, t425: F, t1364: F, t3564: F, t1428: F, t5684: F, t457: F, t1417: F, t2226: F, t2191: F, t442: F, t1056: F, t3544: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5927 = t459 * t2083;
    let t5928 = t5927 * t1175;
    let t5929 = t5926 * t5928;
    let t5932 = t425 * t2083;
    let t5933 = t5932 * t1364;
    let t5934 = t3564 * t5933;
    let t5937 = t1428 * t5684;
    let t5938 = t457 * t5937;
    let t5941 = t1417 * t2226;
    let t5943 = t2191 * t442;
    let t5944 = t5943 * t1056;
    let t5945 = t3544 * t5944;
    (t5927, t5928, t5929, t5932, t5933, t5934, t5937, t5938, t5941, t5944, t5945)
}
