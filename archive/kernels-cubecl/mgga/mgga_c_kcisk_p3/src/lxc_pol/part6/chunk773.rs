//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 773/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk773<F: Float>(t2484: F, t4663: F, t1846: F, t2477: F, t2488: F, t5082: F, t2063: F, t5101: F, t2497: F, t3119: F, t2502: F, t3123: F) -> (F, F, F, F, F, F) {
    let t16037 = t4663 * t2484;
    let t16088 = t1846 * t2477;
    let t16090 = t5082 * t2488;
    let t16099 = t5101 * t2063;
    let t16204 = t3119 * t2497;
    let t16206 = t3123 * t2502;
    (t16037, t16088, t16090, t16099, t16204, t16206)
}
