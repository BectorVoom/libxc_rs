//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1276/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1276(t25577: f64, t3111: f64, t1020: f64, t25576: f64, t1047: f64, t1068: f64, t11653: f64, t11689: f64, t11693: f64, t11707: f64, t11862: f64, t11871: f64, t11930: f64, t25517: f64, t25526: f64, t25580: f64, t27493: f64, t27498: f64, t27536: f64, t3120: f64, t3130: f64, t3136: f64, t3164: f64, t93646: f64, t93649: f64, t93655: f64, t93658: f64, t93667: f64, t93670: f64) -> f64 {
    let t93673 = t25577 * t3111;
    let t93675 = t1020 * t25576;
    let t93678 = 0.14291339372689912324e-2_f64 * t25517 * t11707 + 0.17149607247227894789e-2_f64 * t27536 * t11653 + 0.91464571985215438873e-2_f64 * t93646 * t3130 - 0.13719685797782315831e-1_f64 * t93649 * t1047 - 0.68598428988911579154e-2_f64 * t25526 * t3136 + 0.68598428988911579154e-2_f64 * t93655 * t3164 - 0.25724410870841842183e-2_f64 * t93658 * t11862 + 0.25724410870841842183e-2_f64 * t27493 * t11689 - 0.12862205435420921092e-2_f64 * t27498 * t11693 - 0.12862205435420921092e-2_f64 * t25580 * t11871 + 0.25724410870841842183e-2_f64 * t93667 * t11930 + 0.13719685797782315831e-1_f64 * t93670 * t3120 - 0.60976381323476959248e-2_f64 * t93673 - 0.91464571985215438873e-2_f64 * t93675 * t1068;
    t93678
}
