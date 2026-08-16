//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1420/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1420<F: Float>(t78000: F, t78019: F, t78082: F, t78112: F, t1147: F, t1156: F, t1164: F, t18915: F, t6098: F, t22222: F, t4869: F, t6085: F, t6105: F) -> (F, F, F, F, F) {
    let t78114 = t78000 + t78019 + t78082 + t78112;
    let t78118 = F::cast_from(0.5848223622634646207e0_f64) * t1164 * t1147 * t78114 * t1156;
    let t78120 = F::cast_from(0.70178683471615754484e1_f64) * t18915 * t6098;
    let t78122 = F::cast_from(0.14035736694323150897e2_f64) * t4869 * t22222;
    let t78125 = F::cast_from(0.21053605041484726346e2_f64) * t1164 * t6105 * t6085;
    (t78114, t78118, t78120, t78122, t78125)
}
