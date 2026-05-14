//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 596/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk596<F: Float>(t157: F, t6068: F, t175: F, t398: F, t1772: F, t372: F, t1083: F, t1795: F, t322: F, t1095: F, t384: F, t1165: F, t1879: F, t407: F, t1432: F, t4267: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6069 = t6068 * t157;
    let t6071 = t398 * t175 * t6069;
    let t6074 = t1772 * t372;
    let t6076 = t398 * t1083 * t6074;
    let t6079 = t1795 * t322;
    let t6081 = t398 * t1095 * t6079;
    let t6082 = t384 * t6081;
    let t6086 = t1165 * t1879 * t407;
    let t6090 = t1165 * t4267 * t1432;
    (t6069, t6071, t6074, t6076, t6079, t6081, t6082, t6086, t6090)
}
