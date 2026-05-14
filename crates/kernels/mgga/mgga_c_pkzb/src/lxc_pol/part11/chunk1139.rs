//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1139/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1139<F: Float>(t20982: F, t730: F, t9531: F, t2865: F, t9465: F, t2860: F, t9348: F, t30542: F, t713: F, t722: F, t9245: F, t30758: F, t30761: F, t30764: F, t30767: F, t30769: F, t30772: F) -> (F, F, F, F, F, F) {
    let t30775 = 0.30762056574649219974e4 * t730 * t9531 * t20982;
    let t30778 = 0.35089341735807877242e1 * t730 * t2865 * t9465;
    let t30780 = 0.70178683471615754484e1 * t2860 * t9348;
    let t30784 = 0.5848223622634646207e0 * t730 * t713 * t30542 * t722;
    let t30786 = 0.10526802520742363173e2 * t2860 * t9245;
    let t30787 = -t30758 + t30761 + t30764 - t30767 + t30769 - t30772 - t30775 + t30778 + t30780 - t30784 - t30786;
    (t30775, t30778, t30780, t30784, t30786, t30787)
}
