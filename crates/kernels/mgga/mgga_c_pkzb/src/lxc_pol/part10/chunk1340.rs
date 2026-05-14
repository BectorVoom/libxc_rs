//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1340/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1340<F: Float>(t2156: F, t9716: F, t1306: F, t135: F, t25588: F, t25590: F, t25592: F, t25596: F, t25601: F, t25603: F, t25606: F, t25609: F, t25611: F, t25614: F, t25617: F, t26741: F, t26775: F, t273: F, t803: F, t805: F) -> (F,) {
    let t26780 = t9716 * t2156;
    let t26784 = t135 * t273 * (t26741 + t26775) * t805 - t25588 - 2.0 * t1306 * t26780 * t803 - t25590 - t25592 + t25596 + t25601 + t25603 - t25606 + t25609 + t25611 - t25614 + t25617;
    (t26784,)
}
