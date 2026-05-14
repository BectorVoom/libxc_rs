//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 959/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk959<F: Float>(t2030: F, t35413: F, t4263: F, t2299: F, t7780: F, t3196: F, t33953: F, t13364: F, t31195: F, t7637: F, t8545: F, t1429: F, t7614: F, t1323: F, t361: F, t7436: F) -> (F, F, F, F, F, F, F) {
    let t35415 = t2030 * t35413 * t4263;
    let t35418 = t7780 * t2299;
    let t35420 = t33953 * t3196;
    let t35422 = t31195 * t13364 * t35420;
    let t35425 = t7637 * t8545;
    let t35436 = t7614 * t1429;
    let t35439 = t7436 * t361 * t1323;
    (t35415, t35418, t35420, t35422, t35425, t35436, t35439)
}
