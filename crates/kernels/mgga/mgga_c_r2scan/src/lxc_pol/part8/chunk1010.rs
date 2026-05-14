//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1010/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1010<F: Float>(t88: F, t9904: F, t41: F, t6959: F, t8638: F, t4791: F, t4794: F, t4798: F, t4806: F, t4972: F, t4975: F, t4979: F, t4981: F, t4984: F, t4992: F, t7031: F) -> (F, F, F, F, F, F) {
    let t9905 = t9904 * t88;
    let t9906 = t41 * t9905;
    let t9907 = 0.32530743900905219526e-1 * t6959;
    let t9908 = 3.0 * t8638;
    let t9909 = t4972 - t4975 + t9906 + t9907 - t4979 - t4981 - t4984 + t4791 - t4794 - t4798 + t4806 - t4992 + t9908;
    let t9911 = 3.0 * t7031;
    (t9905, t9906, t9907, t9908, t9909, t9911)
}
