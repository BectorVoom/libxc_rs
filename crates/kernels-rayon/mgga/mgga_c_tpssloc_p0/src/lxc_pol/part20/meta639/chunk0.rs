//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2346/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2346(t1553: f64, t9709: f64, t136: f64, t47763: f64, t908: f64, t47767: f64, t13538: f64, t699: f64, t2826: f64, t47684: f64, t41831: f64, t41833: f64, t41863: f64, t41865: f64, t41870: f64, t41872: f64, t41874: f64, t41876: f64, t48085: f64, t48087: f64, t48090: f64, t48092: f64, t48097: f64, t48098: f64, t48101: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48103 = t9709 * t1553;
    let t48112 = t136 * t908 * t47763;
    let t48114 = t136 * t908 * t47767;
    let t48116 = t699 * t13538;
    let t48119 = t136 * t2826 * t47684;
    let t48120 = 2.0_f64 * t48085 - 2.0_f64 * t48087 - t48090 + t48092 / 6.0_f64 - 10.0_f64 / 9.0_f64 * t41831 - 2.0_f64 / 3.0_f64 * t41833 + t48097 - t48098 / 3.0_f64 + t48101 / 6.0_f64 - 40.0_f64 / 81.0_f64 * t48103 - 40.0_f64 / 27.0_f64 * t41863 + 2.0_f64 / 9.0_f64 * t41865 + 5.0_f64 / 9.0_f64 * t41870 + 5.0_f64 / 27.0_f64 * t41872 - t41874 / 9.0_f64 - 4.0_f64 / 81.0_f64 * t41876 - t48112 - t48114 / 3.0_f64 - 4.0_f64 / 27.0_f64 * t48116 - t48119;
    (t48103, t48112, t48114, t48116, t48119, t48120)
}
