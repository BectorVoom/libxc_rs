//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1192/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1192<F: Float>(t1055: F, t21677: F, t345: F, t1049: F, t5652: F, t21615: F, t21620: F, t1713: F, t922: F, t13761: F, t1734: F, t3132: F) -> (F, F, F, F, F, F, F, F) {
    let t21679 = t345 * t1055 * t21677;
    let t21681 = t1049 * t5652;
    let t21684 = t345 * t1055 * t21615;
    let t21687 = t345 * t1055 * t21620;
    let t21689 = t1713 * t922;
    let t21691 = t345 * t13761 * t21689;
    let t21693 = t1734 * t922;
    let t21695 = t345 * t3132 * t21693;
    (t21679, t21681, t21684, t21687, t21689, t21691, t21693, t21695)
}
