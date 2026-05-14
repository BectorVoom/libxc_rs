//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1151/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1151<F: Float>(t13345: F, t13347: F, t13370: F, t13372: F, t13374: F, t13376: F, t13379: F, t13382: F, t17080: F, t17107: F, t9502: F, t9577: F, t17110: F, t17113: F, t17131: F, t17133: F, t17136: F, t17138: F, t17140: F, t17145: F, t17149: F, t17154: F, t17157: F, t17162: F) -> (F, F) {
    let t17333 = -0.01679259259259259 * t17080 - 0.0008396296296296296 * t13345 - 0.0013993827160493828 * t13347 - 0.006717037037037037 * t13370 + 0.002239012345679012 * t13372 + 0.002518888888888889 * t13374 - 0.010075555555555556 * t13376 + 0.005037777777777778 * t13379 + 0.002518888888888889 * t13382 - 0.0016792592592592592 * t9577 - t9502 - 0.04534 * t17107;
    let t17347 = 0.04534 * t17110 - 0.02518888888888889 * t17113 - 0.05541555555555556 * t17131 + 0.005037777777777778 * t17133 + 0.011335 * t17136 + 0.002518888888888889 * t17138 - 0.0008396296296296296 * t17140 + 0.002518888888888889 * t17145 + 0.0012594444444444445 * t17149 + 0.002099074074074074 * t17154 - 0.007556666666666666 * t17157 - 0.003778333333333333 * t17162;
    (t17333, t17347)
}
