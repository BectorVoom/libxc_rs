//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1771/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1771<F: Float>(t1358: F, t22836: F, t22690: F, t3787: F, t22852: F, t3792: F, t80786: F, t22705: F, t236: F, t3850: F, t550: F, t1361: F, t22792: F, t3719: F) -> (F, F, F, F, F) {
    let t80796 = t22836 * t1358;
    let t80798 = t22690 * t3787;
    let t80801 = t22852 * t80798 * t80786 * t3792;
    let t80807 = t22852 * t22705 * t236 * t3850 * t550;
    let t80814 = t22792 * t22690 * t1361 * t3719;
    (t80796, t80798, t80801, t80807, t80814)
}
