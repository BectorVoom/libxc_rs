//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 915/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk915<F: Float>(t4791: F, t4794: F, t4798: F, t4806: F, t4972: F, t4975: F, t4979: F, t4981: F, t4992: F, t6961: F, t8559: F, t8560: F, t8592: F, t8634: F, t8636: F, t8638: F) -> (F,) {
    let t8639 = t4972 - t4975 + t8559 + t8560 - t4979 + t4981 + t6961 + t8592 + t4791 - t4794 - t4798 + t4806 - t8634 - t4992 + t8636 + t8638;
    (t8639,)
}
