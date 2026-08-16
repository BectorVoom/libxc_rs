//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1054/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1054<F: Float>(t1021: F, t13314: F, t1092: F, t3220: F, t4999: F, t1747: F, t3225: F, t3229: F, t1749: F, t3237: F, t303: F, t4984: F, t922: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t13315 = t1021 * t13314;
    let t13316 = t1092 * t13315;
    let t13318 = t4999 * t3220;
    let t13319 = t1092 * t13318;
    let t13321 = t1747 * t3225;
    let t13322 = t13321 * sigma0;
    let t13323 = t13322 * t3229;
    let t13324 = t1092 * t13323;
    let t13326 = t1749 * t3237;
    let t13327 = t303 * t13326;
    let t13330 = t4984 * t922;
    (t13316, t13319, t13321, t13324, t13327, t13330)
}
