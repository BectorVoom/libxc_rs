//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1145/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1145(t187: f64, t268: f64, t39322: f64, t39347: f64, t39336: f64, t761: f64, t39488: f64, t2374: f64, t39519: f64, t39503: f64, t39391: f64, t39537: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40712 = t187 * t268;
    let t40714 = 0.1301229756036208781e0_f64 * t40712 * t39322;
    let t40716 = 0.19263893255070628431e1_f64 * t40712 * t39347;
    let t40721 = 0.21053605041484726346e2_f64 * t761 * t39336;
    let t40732 = 0.6233709278045326953e3_f64 * t761 * t39488;
    let t40741 = 0.43374325201206959368e-1_f64 * t2374 * t39519;
    let t40743 = 0.12842595503380418954e1_f64 * t2374 * t39503;
    let t40748 = 0.35089341735807877242e1_f64 * t761 * t39391;
    let t40760 = 0.12304822629859687989e5_f64 * t761 * t39537;
    (t40714, t40716, t40721, t40732, t40741, t40743, t40748, t40760)
}
