//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 630/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk630<F: Float>(t13589: F, t6111: F, t2610: F, t3614: F, t2365: F, t2033: F, t11845: F, t959: F, t13555: F, t1457: F, t2103: F, t11724: F, t935: F, t1445: F, t813: F, t3470: F, t3651: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13590 = t6111 * t13589;
    let t13591 = 0.29792074959875355558e-1 * t13590;
    let t13592 = t2610 * t3614;
    let t13593 = t2365 * t13592;
    let t13594 = t2033 * t13593;
    let t13595 = 0.14896037479937677779e-1 * t13594;
    let t13596 = t11845 * t959;
    let t13597 = 0.14896037479937677779e-1 * t13596;
    let t13598 = t1457 * t13555;
    let t13600 = 0.71500979903700853338e0 * t2103 * t13598;
    let t13601 = t11724 * t935;
    let t13602 = t1445 * t13601;
    let t13604 = 0.92023022289409799224e1 * t813 * t13602;
    let t13606 = 0.25025342966295298669e1 * t3651 * t3470;
    (t13591, t13592, t13593, t13595, t13597, t13598, t13600, t13601, t13602, t13604, t13606)
}
