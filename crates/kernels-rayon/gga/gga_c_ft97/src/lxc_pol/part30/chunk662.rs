//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 662/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk662(t24330: f64, t6249: f64, t7012: f64, t1196: f64, t820: f64, t231: f64, t6045: f64, t27569: f64, t6256: f64, t25049: f64, t25070: f64, t25132: f64, t27506: f64, t27507: f64, t27512: f64, t27662: f64, t28584: f64, t28587: f64, t28591: f64, t28595: f64, t28600: f64, t28603: f64, t4094: f64, t6242: f64, t6243: f64, t6250: f64, t6976: f64) -> (f64, f64, f64) {
    let t28613 = t6249 * t24330 * t7012;
    let t28615 = t1196 * t820;
    let t28616 = t231 * t28615;
    let t28617 = t6045 * t28616;
    let t28620 = t6256 * t27569;
    let t28626 = -0.10001700163888888889e0_f64 * t6242 * t28584 + 0.10001700163888888889e0_f64 * t6249 * t6045 * t28587 + t25132 + 0.45306850413028723348e0_f64 * t28591 * t6976 + 0.45306850413028723348e0_f64 * t4094 * t28595 - 0.33339000546296296297e-1_f64 * t25070 * t28600 + 0.40279602951224778277e-1_f64 * t28603 * t27662 + 0.26671200437037037038e0_f64 * t6242 * t27506 * t6243 - 0.26671200437037037038e0_f64 * t6249 * t27506 * t6250 + 0.33339000546296296297e-1_f64 * t28613 + 0.20003400327777777778e0_f64 * t25049 * t28617 - 0.55565000910493827163e-2_f64 * t28620 + 0.33339000546296296298e-1_f64 * t6256 * t27512 + 0.4445200072839506173e-1_f64 * t6256 * t27507;
    (t28615, t28616, t28626)
}
