//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1108/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1108<F: Float>(t1541: F, t57: F, t514: F, t6523: F, t5100: F, t5116: F, t2183: F, t2597: F, t2102: F, t572: F, t565: F, t2141: F, t3433: F, t120: F, t135: F, t6329: F) -> (F, F, F, F, F, F, F) {
    let t19839 = t57 * t1541;
    let t19841 = t514 * t19839 * t6523;
    let t19843 = t5100 * t5116;
    let t19845 = t2183 * t2597;
    let t19851 = t572 * t2102;
    let t19852 = t565 * t19851;
    let t19853 = t3433 * t2141;
    let t19854 = t19852 * t19853;
    let t19858 = 0.49323117337212473701e1 * t120 * t6329 * t135;
    (t19841, t19843, t19845, t19851, t19852, t19854, t19858)
}
