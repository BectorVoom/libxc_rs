//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 620/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk620<F: Float>(t2731: F, t566: F, t378: F, t5: F, t966: F, t750: F, t963: F, t741: F, t1859: F, t897: F) -> (F, F, F, F, F) {
    let t2732 = t566 * t2731;
    let t2736 = t5 * t378 * t966;
    let t2738 = t963 * t750;
    let t2741 = t963 * t741;
    let t2743 = t1859 * t897;
    (t2732, t2736, t2738, t2741, t2743)
}
