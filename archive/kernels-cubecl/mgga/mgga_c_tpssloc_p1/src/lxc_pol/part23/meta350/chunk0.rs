//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1145/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1145<F: Float>(t187: F, t268: F, t39322: F, t39347: F, t39336: F, t761: F, t39488: F, t2374: F, t39519: F, t39503: F, t39391: F, t39537: F) -> (F, F, F, F, F, F, F, F) {
    let t40712 = t187 * t268;
    let t40714 = F::cast_from(0.1301229756036208781e0_f64) * t40712 * t39322;
    let t40716 = F::cast_from(0.19263893255070628431e1_f64) * t40712 * t39347;
    let t40721 = F::cast_from(0.21053605041484726346e2_f64) * t761 * t39336;
    let t40732 = F::cast_from(0.6233709278045326953e3_f64) * t761 * t39488;
    let t40741 = F::cast_from(0.43374325201206959368e-1_f64) * t2374 * t39519;
    let t40743 = F::cast_from(0.12842595503380418954e1_f64) * t2374 * t39503;
    let t40748 = F::cast_from(0.35089341735807877242e1_f64) * t761 * t39391;
    let t40760 = F::cast_from(0.12304822629859687989e5_f64) * t761 * t39537;
    (t40714, t40716, t40721, t40732, t40741, t40743, t40748, t40760)
}
