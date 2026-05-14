//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1067/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1067<F: Float>(t11905: F, t5248: F, t7715: F, t18147: F, t1919: F, t2063: F, t695: F, t8662: F, t1060: F, t2543: F, t707: F, t11894: F, t11900: F, t1470: F, t18063: F, t18089: F, t18155: F, t18156: F, t1909: F, t22530: F, t22668: F, t22933: F, t22944: F, t5231: F, t7051: F, t7056: F, t7060: F, t725: F, t8915: F, t8919: F, t8923: F, t8927: F) -> (F,) {
    let t24405 = t5248 * t11905 * t7715;
    let t24428 = t1919 * t18147 * t2063;
    let t24434 = t8662 * t695;
    let t24436 = t1919 * t24434 * t1060;
    let t24439 = t2543 * t707;
    let t24442 = -0.44218518518518518518e-1 * t1470 * t24405 - 0.1857375e-1 * t5231 * t22530 + 0.88437037037037037037e-2 * t11894 - 0.1857375e-1 * t11900 * t8919 - 0.123825e-1 * t2543 * t7060 + 0.46434375e-2 * t1909 * t8915 + 0.9286875e-2 * t1909 * t8923 + 0.9286875e-2 * t725 * t22944 + 0.123825e-1 * t1909 * t8927 + 0.1857375e-1 * t5231 * t22668 - 0.232171875e-2 * t18063 * t22933 - 0.53062222222222222222e-1 * t1470 * t24428 - 0.1857375e-1 * t18089 * t7051 - t18155 + 0.70749629629629629628e-1 * t18156 - 0.26531111111111111111e-1 * t1470 * t24436 + 0.24765e-1 * t24439 * t7056;
    (t24442,)
}
