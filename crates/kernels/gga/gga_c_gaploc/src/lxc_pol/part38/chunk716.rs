//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 716/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk716<F: Float>(t13596: F, t13555: F, t1457: F, t2103: F, t11724: F, t935: F, t1445: F, t813: F, t3470: F, t3651: F, t11798: F, t1645: F, t2624: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13597 = F::new(0.14896037479937677779e-1) * t13596;
    let t13598 = t1457 * t13555;
    let t13600 = F::new(0.71500979903700853338e0) * t2103 * t13598;
    let t13601 = t11724 * t935;
    let t13602 = t1445 * t13601;
    let t13604 = F::new(0.92023022289409799224e1) * t813 * t13602;
    let t13606 = F::new(0.25025342966295298669e1) * t3651 * t3470;
    let t13608 = F::new(0.10725146985555128001e1) * t11798 * t3470;
    let t13609 = t1645 * t2624;
    (t13597, t13598, t13600, t13601, t13602, t13604, t13606, t13608, t13609)
}
