//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3215/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3215<F: Float>(t1269: F, t24543: F, t24704: F, t3153: F, t24751: F, t1248: F, t12717: F, t12751: F, t12756: F, t1285: F, t1287: F, t13142: F, t13143: F, t21495: F, t21535: F, t24864: F, t24998: F, t5436: F, t5458: F, t5465: F, t5478: F, t5480: F, t59730: F, t70311: F, t72329: F, t72724: F, t73: F, t82775: F, t82859: F, t82886: F) -> (F, F, F) {
    let t84462 = t1269 * t24543;
    let t84487 = t24704 * t3153;
    let t84495 = t24751 * t3153;
    let t84506 = -F::cast_from(0.39512695097613069591e1_f64) * t13142 * t84462 * t13143 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t24864 * t1248 * t1287 + F::cast_from(0.19756347548806534796e1_f64) * t12756 * t72329 * t82775 + F::cast_from(0.19756347548806534796e1_f64) * t5436 * t21495 - F::cast_from(0.65854491829355115987e0_f64) * t5478 * t82859 * t5480 + F::cast_from(0.92196288561097162379e1_f64) * t59730 * t82886 * t72724 * t1248 + F::cast_from(0.19756347548806534796e1_f64) * t5436 * t21535 - F::cast_from(0.19756347548806534796e1_f64) * t5478 * t70311 * t24998 + F::cast_from(0.19756347548806534796e1_f64) * t12756 * t84487 * t5480 + F::cast_from(0.39512695097613069592e1_f64) * t12717 * t24751 * t73 * t5458 - F::cast_from(0.39512695097613069592e1_f64) * t12751 * t84495 * t5465 + F::cast_from(0.19756347548806534796e1_f64) * t12756 * t84495 * t5480 + F::cast_from(0.39512695097613069592e1_f64) * t12717 * t24704 * t73 * t5458;
    (t84462, t84487, t84506)
}
