//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 877/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk877<F: Float>(t1979: F, t3604: F, t721: F, t2848: F, t2852: F, t5522: F, t5758: F, t7357: F, t7508: F, t9148: F, t9163: F, t261: F) -> (F, F, F, F, F) {
    let t9451 = t3604 * t1979;
    let t9452 = t9451 * t721;
    let t9455 = t2852 * t2848;
    let t9462 = -t5758 + F::new(0.12361111111111111111e-1) * t5522 + F::new(0.24722222222222222223e-1) * t7357 - t7508 - F::new(0.92708333333333333333e-2) * t9148 + F::new(0.278125e-1) * t9163;
    let t9463 = t9462 * t261;
    (t9451, t9452, t9455, t9462, t9463)
}
