//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 917/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk917<F: Float>(t2188: F, t8611: F, t2189: F, t3356: F, t6579: F, t2236: F, t3352: F, t809: F, t2234: F, t2228: F, t1346: F, t6564: F, t6562: F, t1179: F, t6536: F, t2170: F, t3: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8613 = 2.0 * t2188 * t8611;
    let t8614 = t3356 * t2189;
    let t8616 = 0.96491876992155210402e2 * t6579 * t8614;
    let t8617 = t3352 * t2236;
    let t8618 = t8617 * t809;
    let t8620 = 0.32163958997385070134e2 * t2234 * t8618;
    let t8621 = t3356 * t2228;
    let t8623 = 0.16081979498692535067e2 * t2234 * t8621;
    let t8624 = t1346 * t6564;
    let t8625 = t8624 * t2189;
    let t8627 = 0.51726012919273400301e3 * t6562 * t8625;
    let t8632 = t6536 * t1179;
    let t8635 = t2170 * t3;
    (t8613, t8614, t8616, t8618, t8620, t8621, t8623, t8625, t8627, t8632, t8635)
}
