//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2172/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2172(t25277: f64, t4458: f64, t14685: f64, t14756: f64, t7021: f64, t14760: f64, t93015: f64, t93067: f64, t93069: f64, t93073: f64, t93077: f64, t93080: f64, t93084: f64, t93086: f64, t93088: f64, t93091: f64, t93095: f64) -> f64 {
    let t99099 = t25277 * t4458;
    let t99100 = 7.0_f64 / 72.0_f64 * t99099;
    let t99102 = t7021 * t14685 * t14756;
    let t99103 = 7.0_f64 / 24.0_f64 * t99102;
    let t99113 = t93015 * t14760;
    let t99116 = t99100 - t99103 - 0.90702367218671976886e-1_f64 * t93067 + 0.80031500487063509016e-2_f64 * t93069 + 0.2168320119862840671e-2_f64 * t93073 - 0.10164000561857065645e-3_f64 * t93077 + 0.14291339372689912324e-4_f64 * t93080 - 0.28582678745379824648e-4_f64 * t93084 - 0.40015750243531754508e-1_f64 * t93086 - 0.30488190661738479624e-3_f64 * t93088 + 0.14291339372689912324e-4_f64 * t93091 - 0.90357964994909313586e-5_f64 * t99113 + 0.50820002809285328225e-3_f64 * t93095;
    t99116
}
