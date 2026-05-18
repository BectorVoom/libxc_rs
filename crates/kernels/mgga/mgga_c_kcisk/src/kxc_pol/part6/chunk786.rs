//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 786/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk786<F: Float>(t1248: F, t13614: F, t2075: F, t2115: F, t4030: F, t4080: F, t2201: F, t3119: F, t2206: F, t3123: F, t2198: F, t3114: F) -> (F, F, F, F, F, F) {
    let t20373 = t1248 * t13614 * t2075;
    let t20552 = t2115 * t4030;
    let t20567 = t2115 * t4080;
    let t20752 = t3119 * t2201;
    let t20754 = t3123 * t2206;
    let t20763 = t3114 * t2198;
    (t20373, t20552, t20567, t20752, t20754, t20763)
}
