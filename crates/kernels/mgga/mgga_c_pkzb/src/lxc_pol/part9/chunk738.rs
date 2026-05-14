//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 738/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk738<F: Float>(t133: F, t1773: F, t5181: F, t568: F, t614: F, t1692: F, t5217: F, t596: F, t160: F, t162: F, t1742: F, t1747: F, t1750: F, t2631: F, t5348: F, t594: F, t597: F) -> (F, F, F, F) {
    let t5356 = t133 * t1773;
    let t5357 = t5356 * t5181;
    let t5360 = t614 * t568;
    let t5361 = t5360 * t1692;
    let t5364 = t596 * t5217;
    let t5367 = 60.0 * t160 * t5357 + 3.0 * t160 * t5364 - t162 * t5348 + 9.0 * t1742 * t597 - 36.0 * t1747 * t594 + 9.0 * t1750 * t594 - 36.0 * t2631 * t5361;
    (t5357, t5361, t5364, t5367)
}
