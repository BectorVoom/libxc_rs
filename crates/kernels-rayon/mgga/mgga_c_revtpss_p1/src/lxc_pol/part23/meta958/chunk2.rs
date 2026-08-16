//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3215/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3215(t1269: f64, t24543: f64, t24704: f64, t3153: f64, t24751: f64, t1248: f64, t12717: f64, t12751: f64, t12756: f64, t1285: f64, t1287: f64, t13142: f64, t13143: f64, t21495: f64, t21535: f64, t24864: f64, t24998: f64, t5436: f64, t5458: f64, t5465: f64, t5478: f64, t5480: f64, t59730: f64, t70311: f64, t72329: f64, t72724: f64, t73: f64, t82775: f64, t82859: f64, t82886: f64) -> (f64, f64, f64) {
    let t84462 = t1269 * t24543;
    let t84487 = t24704 * t3153;
    let t84495 = t24751 * t3153;
    let t84506 = -0.39512695097613069591e1_f64 * t13142 * t84462 * t13143 + 0.65854491829355115987e0_f64 * t1285 * t24864 * t1248 * t1287 + 0.19756347548806534796e1_f64 * t12756 * t72329 * t82775 + 0.19756347548806534796e1_f64 * t5436 * t21495 - 0.65854491829355115987e0_f64 * t5478 * t82859 * t5480 + 0.92196288561097162379e1_f64 * t59730 * t82886 * t72724 * t1248 + 0.19756347548806534796e1_f64 * t5436 * t21535 - 0.19756347548806534796e1_f64 * t5478 * t70311 * t24998 + 0.19756347548806534796e1_f64 * t12756 * t84487 * t5480 + 0.39512695097613069592e1_f64 * t12717 * t24751 * t73 * t5458 - 0.39512695097613069592e1_f64 * t12751 * t84495 * t5465 + 0.19756347548806534796e1_f64 * t12756 * t84495 * t5480 + 0.39512695097613069592e1_f64 * t12717 * t24704 * t73 * t5458;
    (t84462, t84487, t84506)
}
