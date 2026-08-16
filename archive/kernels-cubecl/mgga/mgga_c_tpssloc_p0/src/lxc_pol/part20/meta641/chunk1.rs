//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2349/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2349<F: Float>(t48157: F, t13543: F, t699: F, t13547: F, t13556: F, t13529: F, t13533: F, t41887: F, t41889: F, t43002: F, t48122: F, t48125: F, t48128: F, t48131: F, t48134: F, t48137: F, t48142: F, t48145: F, t48148: F, t48153: F, t48156: F) -> (F, F, F, F, F, F) {
    let t48158 = F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t48157;
    let t48159 = t699 * t13543;
    let t48161 = t699 * t13547;
    let t48163 = t699 * t13556;
    let t48165 = t699 * t13529;
    let t48167 = t699 * t13533;
    let t48169 = -F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t48122 + F::cast_from(3.0_f64) * t48125 + t48128 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t48131 + t48134 / F::cast_from(18.0_f64) + F::cast_from(14.0_f64) / F::cast_from(81.0_f64) * t48137 - t48142 + F::cast_from(3.0_f64) * t48145 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t48148 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t41887 - t41889 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) * t48153 - t48156 + t48158 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t48159 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t48161 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t48163 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t48165 - t48167 / F::cast_from(9.0_f64) - t43002;
    (t48159, t48161, t48163, t48165, t48167, t48169)
}
