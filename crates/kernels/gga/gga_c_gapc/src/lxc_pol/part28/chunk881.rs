//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 881/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk881<F: Float>(t3065: F, t3688: F, t3071: F, t474: F, t1030: F, t3076: F, t11326: F, t3144: F, t1971: F, t3044: F, t1743: F, t1912: F, t189: F, t195: F) -> (F, F, F, F, F, F, F, F) {
    let t11471 = t3688 * t3065;
    let t11473 = t474 * t3071;
    let t11474 = t1030 * t11473;
    let t11475 = t11474 * t3076;
    let t11477 = t11326 * t3144;
    let t11479 = t1971 * t3044;
    let t11481 = t1743 * t11479 * t1912;
    let t11483 = t189 * t195;
    (t11471, t11473, t11474, t11475, t11477, t11479, t11481, t11483)
}
