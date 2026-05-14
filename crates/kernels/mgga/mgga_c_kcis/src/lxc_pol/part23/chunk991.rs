//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 991/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk991<F: Float>(t27387: F, t5667: F, t1394: F, t5637: F, t7923: F, t1598: F, t16744: F, t1014: F, t8168: F, t7904: F, t8144: F, t8151: F, t2243: F, t5870: F, t303: F, t1458: F, t8175: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28450 = t27387 * t5667;
    let t28451 = t1394 * t28450;
    let t28453 = t7923 * t5637;
    let t28454 = t1394 * t28453;
    let t28461 = t16744 * t1598;
    let t28465 = t1014 * t8168;
    let t28467 = t8144 * t7904;
    let t28471 = t8151 * t7904;
    let t28473 = t5870 * t2243;
    let t28474 = t303 * t28473;
    let t28476 = t1458 * t8175;
    (t28450, t28451, t28453, t28454, t28461, t28465, t28467, t28471, t28473, t28474, t28476)
}
