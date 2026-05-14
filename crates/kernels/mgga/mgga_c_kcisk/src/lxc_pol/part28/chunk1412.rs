//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1412/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1412<F: Float>(t17775: F, t34306: F, t34374: F, t7293: F, t5218: F, t7444: F, t9988: F, t1907: F, t35262: F, t1957: F, t24081: F, t9718: F, t2799: F, t65168: F, t11701: F, t8968: F) -> (F, F, F, F, F, F, F) {
    let t122367 = 4.0 * t17775 * t34306;
    let t122369 = 2.0 * t7293 * t34374;
    let t122372 = 4.0 * t5218 * t9988 * t7444;
    let t122373 = t35262 * t1907;
    let t122374 = t122373 * t1957;
    let t122375 = t24081 * t9718;
    let t122376 = t65168 * t2799;
    let t122379 = 6.0 * t11701 * t9718 * t8968;
    (t122367, t122369, t122372, t122374, t122375, t122376, t122379)
}
