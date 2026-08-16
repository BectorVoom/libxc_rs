//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1047/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1047<F: Float>(t35662: F, t35664: F, t35733: F, t35738: F, t35740: F, t35744: F, t35790: F, t35818: F, t35829: F, t35882: F, t35885: F, t35924: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37652 = F::cast_from(0.45017719023973223821e-1_f64) * t35662;
    let t37653 = F::cast_from(0.22675591804667994221e-1_f64) * t35664;
    let t37694 = F::cast_from(0.17149607247227894789e-2_f64) * t35733;
    let t37697 = F::cast_from(0.68598428988911579156e-2_f64) * t35738;
    let t37698 = F::cast_from(0.16006300097412701803e-1_f64) * t35740;
    let t37700 = F::cast_from(0.25724410870841842184e-2_f64) * t35744;
    let t37719 = F::cast_from(0.17149607247227894789e-2_f64) * t35790;
    let t37733 = F::cast_from(0.28582678745379824648e-3_f64) * t35818;
    let t37736 = F::cast_from(0.16006300097412701803e-1_f64) * t35829;
    let t37757 = t35882 / F::cast_from(64.0_f64);
    let t37758 = t35885 / F::cast_from(192.0_f64);
    let t37786 = F::cast_from(13.0_f64) / F::cast_from(144.0_f64) * t35924;
    (t37652, t37653, t37694, t37697, t37698, t37700, t37719, t37733, t37736, t37757, t37758, t37786)
}
