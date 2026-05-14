//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 797/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk797<F: Float>(t1145: F, t4544: F, t2880: F, t4524: F, t4530: F, t521: F, t2874: F, t4540: F, t1139: F, t513: F, t1128: F, t502: F, t535: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4545 = t1145 * t4544;
    let t4550 = t2880 * t4524;
    let t4553 = t521 * t4530;
    let t4556 = t2874 * t4524;
    let t4559 = t521 * t4540;
    let t4562 = t1139 * t4544;
    let t4565 = t513 * t4540;
    let t4568 = t1128 * t4544;
    let t4571 = t502 * t4530;
    let t4574 = t535 * t4530;
    (t4545, t4550, t4553, t4556, t4559, t4562, t4565, t4568, t4571, t4574)
}
