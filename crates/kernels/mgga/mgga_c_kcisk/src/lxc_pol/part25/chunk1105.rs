//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1105/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1105<F: Float>(t32995: F, t9664: F, t5195: F, t9679: F, t1799: F, t11197: F, t648: F, t1772: F) -> (F, F, F, F, F) {
    let t32996 = t9664 * t32995;
    let t32998 = t9679 * t5195;
    let t32999 = t1799 * t32998;
    let t33001 = t11197 * t648;
    let t33002 = t33001 * t1772;
    (t32996, t32998, t32999, t33001, t33002)
}
