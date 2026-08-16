//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1035/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1035<F: Float>(t30979: F, t3482: F, t30852: F, t5625: F, t3484: F, t5634: F, t5633: F, t30184: F, t3796: F, t2173: F, t26992: F, t13293: F, t30189: F) -> (F, F, F, F, F, F) {
    let t30980 = t3482 * t30979;
    let t30982 = t5625 * t30852;
    let t30983 = t3484 * t30982;
    let t30984 = t3482 * t30983;
    let t30986 = t5634 * t30852;
    let t30987 = t3484 * t30986;
    let t30988 = t5633 * t30987;
    let t30990 = t5634 * t30184;
    let t30991 = t3796 * t30990;
    let t30992 = t5633 * t30991;
    let t30994 = t26992 * t2173;
    let t31000 = t13293 * t30189;
    (t30980, t30984, t30988, t30992, t30994, t31000)
}
