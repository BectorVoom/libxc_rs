//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 662/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk662<F: Float>(t24330: F, t6249: F, t7012: F, t1196: F, t820: F, t231: F, t6045: F, t27569: F, t6256: F, t25049: F, t25070: F, t25132: F, t27506: F, t27507: F, t27512: F, t27662: F, t28584: F, t28587: F, t28591: F, t28595: F, t28600: F, t28603: F, t4094: F, t6242: F, t6243: F, t6250: F, t6976: F) -> (F, F, F) {
    let t28613 = t6249 * t24330 * t7012;
    let t28615 = t1196 * t820;
    let t28616 = t231 * t28615;
    let t28617 = t6045 * t28616;
    let t28620 = t6256 * t27569;
    let t28626 = -F::cast_from(0.10001700163888888889e0_f64) * t6242 * t28584 + F::cast_from(0.10001700163888888889e0_f64) * t6249 * t6045 * t28587 + t25132 + F::cast_from(0.45306850413028723348e0_f64) * t28591 * t6976 + F::cast_from(0.45306850413028723348e0_f64) * t4094 * t28595 - F::cast_from(0.33339000546296296297e-1_f64) * t25070 * t28600 + F::cast_from(0.40279602951224778277e-1_f64) * t28603 * t27662 + F::cast_from(0.26671200437037037038e0_f64) * t6242 * t27506 * t6243 - F::cast_from(0.26671200437037037038e0_f64) * t6249 * t27506 * t6250 + F::cast_from(0.33339000546296296297e-1_f64) * t28613 + F::cast_from(0.20003400327777777778e0_f64) * t25049 * t28617 - F::cast_from(0.55565000910493827163e-2_f64) * t28620 + F::cast_from(0.33339000546296296298e-1_f64) * t6256 * t27512 + F::cast_from(0.4445200072839506173e-1_f64) * t6256 * t27507;
    (t28615, t28616, t28626)
}
