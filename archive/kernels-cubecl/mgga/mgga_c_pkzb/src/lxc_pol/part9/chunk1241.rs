//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1241/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1241<F: Float>(t17946: F, t21454: F, t287: F, t5726: F, t2104: F, t5974: F, t7719: F, t7649: F, t2922: F, t7654: F, t774: F, t7659: F) -> (F, F, F, F, F, F) {
    let t21729 = t17946 * t21454;
    let t21730 = t5726 * t287;
    let t21746 = t2104 * t5974 * t7719;
    let t21749 = t2104 * t5974 * t7649;
    let t21752 = t2922 * t774 * t7654;
    let t21755 = t2922 * t774 * t7659;
    (t21729, t21730, t21746, t21749, t21752, t21755)
}
