//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1231/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1231(t185: f64, t2658: f64, t39103: f64, t607: f64, t707: f64, t9862: f64, t2250: f64, t4194: f64, t750: f64, t39658: f64, t41266: f64, t41270: f64, t41273: f64, t41275: f64, t41278: f64, t41281: f64, t41283: f64, t41286: f64) -> (f64, f64, f64, f64) {
    let t41289 = 36.0_f64 * t2658 * t185 * t39103;
    let t41291 = t707 * t9862 * t607;
    let t41292 = 16.0_f64 * t41291;
    let t41295 = t4194 * t750 * t607 * t2250;
    let t41296 = 144.0_f64 * t41295;
    let t41297 = -t41266 + t41270 - t39658 + t41273 + t41275 + t41278 + t41281 + t41283 + t41286 + t41289 + t41292 + t41296;
    (t41289, t41292, t41296, t41297)
}
