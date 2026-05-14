//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1386/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1386<F: Float>(t22023: F, t22030: F, t22034: F, t22036: F, t22039: F, t22041: F, t22045: F, t22086: F, t22089: F, t26860: F, t26862: F, t28885: F, t28887: F, t28890: F, t18786: F, t18839: F, t18843: F, t18855: F, t23320: F, t23321: F, t23694: F, t246: F, t26873: F, t28922: F, t32071: F, t32131: F) -> (F, F) {
    let t33709 = -0.300153217574e-2 * t28885 - 0.96319466275353142158e0 * t28887 - 0.600306435148e-2 * t28890 - t22023 - t22030 + t22034 + t22036 + t22039 - t22041 - t22045 + t22086 + t22089 + 36.0 * t26860 + t26862;
    let t33722 = t18786 - t32071 + t23320 + t23321 + t18839 - t18843 - 0.285764e-1 * t246 * t32131 + t18855 - 0.4051561992e0 * t28922 + t23694 + 0.5143752e0 * t26873;
    (t33709, t33722)
}
