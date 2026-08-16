//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2266/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2266<F: Float>(t18469: F, t3447: F, t44525: F, t18206: F, t52133: F, t4899: F, t6138: F, t6144: F, t15376: F, t15420: F, t15419: F, t18211: F) -> (F, F, F, F, F, F) {
    let t64627 = t3447 * t44525 * t18469;
    let t64632 = t3447 * t52133 * t18206;
    let t64644 = t4899 * t6138;
    let t64648 = t4899 * t6144;
    let t64667 = t15376 * t15420;
    let t64686 = t3447 * t15419 * t18211;
    (t64627, t64632, t64644, t64648, t64667, t64686)
}
