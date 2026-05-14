//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1202/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1202<F: Float>(t7736: F, t7738: F, t774: F, t7742: F, t7744: F, t17867: F, t2104: F, t2911: F, t2064: F, t2922: F, t2924: F, t5974: F, t7672: F, t7677: F, t2899: F, t7682: F) -> (F, F, F, F, F, F, F) {
    let t21617 = t7736 * t774 * t7738;
    let t21620 = t7742 * t774 * t7744;
    let t21623 = t2104 * t17867 * t2911;
    let t21626 = t2922 * t2064 * t2924;
    let t21633 = t2104 * t5974 * t7672;
    let t21637 = t2104 * t5974 * t7677;
    let t21640 = t2899 * t5974 * t7682;
    (t21617, t21620, t21623, t21626, t21633, t21637, t21640)
}
