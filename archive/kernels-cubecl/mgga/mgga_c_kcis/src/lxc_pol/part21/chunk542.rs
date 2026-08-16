//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 542/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk542<F: Float>(t1142: F, t3481: F, t20: F, t2865: F, t414: F, t1242: F, t1247: F, t1241: F, t68: F) -> (F, F, F, F, F) {
    let t3482 = t1142 * t3481;
    let t3483 = t2865 * t20;
    let t3484 = t414 * t3483;
    let t3487 = t1242 * t1247;
    let t3489 = t1241 * t68;
    (t3482, t3483, t3484, t3487, t3489)
}
