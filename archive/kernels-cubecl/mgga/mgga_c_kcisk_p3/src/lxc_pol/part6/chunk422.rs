//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 422/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk422<F: Float>(t3063: F, t933: F, t177: F, t918: F, t140: F, t191: F, t119: F, t974: F, t139: F, t172: F, t1003: F, t167: F, t944: F) -> (F, F, F, F, F, F) {
    let t3064 = t3063 * t933;
    let t3069 = t918 * t177;
    let t3071 = t140 * t3069 * t191;
    let t3073 = t119 * t974;
    let t3075 = t140 * t3073 * t191;
    let t3077 = t139 * t172;
    let t3078 = t3077 * t1003;
    let t3082 = t167 * t944;
    (t3064, t3071, t3075, t3077, t3078, t3082)
}
