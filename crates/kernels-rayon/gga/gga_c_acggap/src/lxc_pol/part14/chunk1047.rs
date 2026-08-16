//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1047/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1047(t35662: f64, t35664: f64, t35733: f64, t35738: f64, t35740: f64, t35744: f64, t35790: f64, t35818: f64, t35829: f64, t35882: f64, t35885: f64, t35924: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37652 = 0.45017719023973223821e-1_f64 * t35662;
    let t37653 = 0.22675591804667994221e-1_f64 * t35664;
    let t37694 = 0.17149607247227894789e-2_f64 * t35733;
    let t37697 = 0.68598428988911579156e-2_f64 * t35738;
    let t37698 = 0.16006300097412701803e-1_f64 * t35740;
    let t37700 = 0.25724410870841842184e-2_f64 * t35744;
    let t37719 = 0.17149607247227894789e-2_f64 * t35790;
    let t37733 = 0.28582678745379824648e-3_f64 * t35818;
    let t37736 = 0.16006300097412701803e-1_f64 * t35829;
    let t37757 = t35882 / 64.0_f64;
    let t37758 = t35885 / 192.0_f64;
    let t37786 = 13.0_f64 / 144.0_f64 * t35924;
    (t37652, t37653, t37694, t37697, t37698, t37700, t37719, t37733, t37736, t37757, t37758, t37786)
}
