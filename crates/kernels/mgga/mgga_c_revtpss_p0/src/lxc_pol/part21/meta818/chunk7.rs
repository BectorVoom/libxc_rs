//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3018/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3018<F: Float>(t1041: F, t1670: F, t42994: F, t11922: F, t15786: F, t4892: F, t1042: F, t1063: F, t11779: F, t11933: F, t12160: F, t15780: F, t15787: F, t16020: F, t16040: F, t16048: F, t16052: F, t16067: F, t16068: F, t16104: F, t1665: F, t3117: F, t42340: F, t43066: F, t4806: F, t4854: F, t4899: F, t4902: F, t54450: F, t54479: F) -> F {
    let t55247 = t1041 * t42994 * t1670;
    let t55265 = t4892 * t11922 * t15786;
    let t55271 = F::cast_from(0.23818898954483187207e-3_f64) * t1063 * t1042 * t4806 * t54450 - F::cast_from(0.21722835846488666732e-1_f64) * t11779 * t4854 + F::cast_from(0.63517063878621832551e-4_f64) * t55247 + F::cast_from(0.68598428988911579154e-2_f64) * t12160 * t16048 * t4902 + F::cast_from(0.68598428988911579154e-2_f64) * t11933 * t16040 + F::cast_from(0.45732285992607719436e-2_f64) * t43066 * t16104 - F::cast_from(0.21722835846488666732e-1_f64) * t42340 * t1665 + F::cast_from(0.64311027177104605458e-3_f64) * t16067 * t3117 * t54479 * t16068 - F::cast_from(0.68598428988911579154e-2_f64) * t16052 * t15787 + F::cast_from(0.85748036236139473944e-3_f64) * t55265 - F::cast_from(0.64311027177104605458e-3_f64) * t4899 * t3117 * t15780 * t16020;
    t55271
}
