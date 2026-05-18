//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1087/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1087<F: Float>(t1947: F, t1954: F, t5498: F, t709: F, t1976: F, t5490: F, t1953: F, t1975: F, t252: F, t5749: F, t663: F, t1847: F, t1898: F) -> (F, F, F, F, F, F, F, F) {
    let t17621 = t1947 * t1954;
    let t17624 = t709 * t5498;
    let t17630 = t1947 * t1976;
    let t17633 = t709 * t5490;
    let t17637 = F::new(1.0) / t1975 / t1953;
    let t17638 = t252 * t17637;
    let t17650 = t5749 * t663;
    let t17655 = t1847 * t1898;
    (t17621, t17624, t17630, t17633, t17637, t17638, t17650, t17655)
}
