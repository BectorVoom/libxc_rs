//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 997/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk997<F: Float>(t374: F, t9782: F, t288: F, t2892: F, t481: F, t97: F, t3016: F, t2858: F, t4695: F, t4703: F, t4880: F, t4891: F, t4901: F, t6946: F, t6948: F, t6951: F, t8545: F, t8547: F, t8550: F, t8552: F) -> (F, F, F, F, F, F, F) {
    let t9783 = t9782 * t374;
    let t9786 = t97 * t481 * t288 * t2892;
    let t9787 = 6.0 * t9786;
    let t9788 = t288 * t3016;
    let t9790 = t2858 * t9788 * t481;
    let t9791 = 6.0 * t9790;
    let t9793 = -t4695 - t4880 + t6946 - t8545 + t6948 + t4891 - t4703 - t6951 - t8547 + t8550 + t8552 - t4901;
    (t9783, t9786, t9787, t9788, t9790, t9791, t9793)
}
