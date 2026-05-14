//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 472/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk472<F: Float>(t1254: F, t3500: F, t1251: F, t1263: F, t25: F, t286: F, t2887: F, t2844: F, t421: F, t283: F, t414: F, t990: F) -> (F, F, F, F, F) {
    let t3501 = t3500 * t1254;
    let t3502 = t1251 * t3501;
    let t3504 = t25 * t1263;
    let t3505 = t1251 * t3504;
    let t3507 = t286 * t2887;
    let t3508 = t421 * t2844;
    let t3513 = t414 * t283;
    let t3514 = t3513 * t990;
    (t3502, t3505, t3507, t3508, t3514)
}
