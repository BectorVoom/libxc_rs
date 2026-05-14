//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1234/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1234<F: Float>(t26675: F, t5276: F, t963: F, t5348: F, t5296: F, t41: F, t5844: F, t898: F, t4911: F, t959: F, t21120: F, t7755: F, t21167: F, t5249: F, t970: F, t5836: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26676 = 0.12154685976e1 * t26675;
    let t26680 = t963 * t5276;
    let t26682 = t963 * t5348;
    let t26684 = t963 * t5296;
    let t26687 = t41 * t898 * t5844;
    let t26688 = t4911 * t959;
    let t26690 = t7755 * t21120;
    let t26693 = t5249 * t970 * t21167;
    let t26695 = t898 * t5836;
    (t26676, t26680, t26682, t26684, t26687, t26688, t26690, t26693, t26695)
}
