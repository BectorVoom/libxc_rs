//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 718/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk718<F: Float>(t10443: F, t10457: F, t338: F, t118: F, t558: F, t9523: F, t10168: F, t10170: F, t10177: F, t10179: F, t10181: F, t10183: F, t10185: F, t10252: F, t10257: F, t10260: F, t10350: F, t10353: F, t10387: F, t10417: F, t10420: F, t305: F, t326: F, t4669: F, t793: F, t838: F, t8998: F, t9001: F, t9009: F) -> (F, F, F) {
    let t10458 = t10443 + t10457;
    let t10459 = t338 * t10458;
    let t10460 = t118 * t10459;
    let t10471 = t9523 * t558;
    let t10480 = -F::cast_from(0.13637330827122670865e-1_f64) * t10168 - F::cast_from(0.40911992481368012596e-1_f64) * t10170 - F::cast_from(0.11974241701863808564e0_f64) * t326 * t10417 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t10420 - F::cast_from(0.39914139006212695214e-1_f64) * t118 * t10257 - F::cast_from(0.79828278012425390428e-1_f64) * t118 * t10260 + F::cast_from(0.17961362552795712846e0_f64) * t10177 + F::cast_from(0.19957069503106347607e-1_f64) * t10460 + F::cast_from(0.11974241701863808564e0_f64) * t793 * t10252 + F::cast_from(0.11974241701863808564e0_f64) * t305 * t10350 + F::cast_from(0.59871208509319042821e-1_f64) * t305 * t10353 - F::cast_from(0.5454932330849068346e-1_f64) * t10179 + F::cast_from(0.16364796992547205038e0_f64) * t10181 + F::cast_from(0.40911992481368012596e-1_f64) * t10183 - F::cast_from(0.35922725105591425692e0_f64) * t4669 * t10471 + F::cast_from(0.23948483403727617128e0_f64) * t838 * t10387 + F::cast_from(0.5987120850931904282e-1_f64) * t10185 + F::cast_from(0.15965655602485078085e0_f64) * t8998 - F::cast_from(0.3193131120497015617e0_f64) * t9001 + F::cast_from(0.47896966807455234256e0_f64) * t9009;
    (t10458, t10459, t10480)
}
