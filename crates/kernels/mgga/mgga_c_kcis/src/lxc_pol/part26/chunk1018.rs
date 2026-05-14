//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1018/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1018<F: Float>(t28594: F, t7949: F, t5627: F, t6028: F, t7948: F, t1548: F, t5748: F, t27520: F, t6029: F, t1552: F, t5752: F, t5932: F, t2066: F, t3738: F, t1928: F, t570: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28595 = t28594 * t7949;
    let t28597 = t6028 * t5627;
    let t28598 = t7948 * t28597;
    let t28600 = t5748 * t1548;
    let t28602 = t27520 * t6029;
    let t28604 = t5752 * t1552;
    let t28606 = t7948 * t5932;
    let t28608 = t3738 * t2066;
    let t28610 = t570 * t1928;
    (t28595, t28597, t28598, t28600, t28602, t28604, t28606, t28608, t28610)
}
