//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1365/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1365(t77072: f64, t894: f64, t2798: f64, t77041: f64, t41942: f64, t77075: f64, t42087: f64, t47787: f64, t76587: f64, t76595: f64, t76610: f64, t76618: f64, t76626: f64, t76899: f64, t76903: f64, t76906: f64, t76912: f64) -> (f64, f64, f64, f64) {
    let t77102 = t894 * t77072;
    let t77105 = t2798 * t77041;
    let t77107 = t41942 * t77075;
    let t77114 = t42087 - 0.21908444444444444444e0_f64 * t76899 + 0.65725333333333333332e0_f64 * t76903 - 0.10954222222222222222e0_f64 * t76906 - 0.295764e1_f64 * t76912 + 0.1898925e1_f64 * t77102 + 0.12401580246913580247e1_f64 * t47787 - 0.28483875e1_f64 * t77105 + 0.1151859375e0_f64 * t77107 - 0.19931111111111111111e1_f64 * t76587 + 0.71752000000000000001e1_f64 * t76595 - 0.79724444444444444444e0_f64 * t76610 - 0.107628e2_f64 * t76618 + 0.23917333333333333333e1_f64 * t76626;
    (t77102, t77105, t77107, t77114)
}
