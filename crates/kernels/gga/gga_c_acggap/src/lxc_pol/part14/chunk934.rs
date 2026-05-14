//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 934/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk934<F: Float>(t35587: F, t35643: F, t35662: F, t35664: F, t35733: F, t35738: F, t35740: F, t35744: F, t35790: F, t35818: F, t35829: F, t35882: F, t35885: F, t35924: F, t35936: F, t35938: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37622 = 0.85748036236139473944e-3 * t35587;
    let t37645 = 13.0 / 48.0 * t35643;
    let t37652 = 0.45017719023973223821e-1 * t35662;
    let t37653 = 0.22675591804667994221e-1 * t35664;
    let t37694 = 0.17149607247227894789e-2 * t35733;
    let t37697 = 0.68598428988911579156e-2 * t35738;
    let t37698 = 0.16006300097412701803e-1 * t35740;
    let t37700 = 0.25724410870841842184e-2 * t35744;
    let t37719 = 0.17149607247227894789e-2 * t35790;
    let t37733 = 0.28582678745379824648e-3 * t35818;
    let t37736 = 0.16006300097412701803e-1 * t35829;
    let t37757 = t35882 / 64.0;
    let t37758 = t35885 / 192.0;
    let t37786 = 13.0 / 144.0 * t35924;
    let t37791 = 0.3973125e0 * t35936;
    let t37792 = 0.264875e0 * t35938;
    (t37622, t37645, t37652, t37653, t37694, t37697, t37698, t37700, t37719, t37733, t37736, t37757, t37758, t37786, t37791, t37792)
}
