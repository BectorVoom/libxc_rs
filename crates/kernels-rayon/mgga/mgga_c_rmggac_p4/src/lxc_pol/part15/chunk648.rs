//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 648/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk648(t5259: f64, t8901: f64, t4669: f64, t8905: f64, t2402: f64, t321: f64, t305: f64, t326: f64, t8824: f64, t8866: f64, t8994: f64, t8998: f64, t9001: f64, t9003: f64, t9006: f64, t9009: f64, t9011: f64, t9013: f64, t9015: f64, t9017: f64) -> (f64, f64) {
    let t9021 = t5259 * t8901;
    let t9023 = t4669 * t8905;
    let t9025 = t2402 * t321;
    let t9028 = -0.59871208509319042821e-1_f64 * t326 * t8824 + 0.59871208509319042821e-1_f64 * t305 * t8994 + 0.39914139006212695213e-1_f64 * t8998 - 0.79828278012425390427e-1_f64 * t9001 + 0.2993560425465952141e-1_f64 * t9003 + 0.2993560425465952141e-1_f64 * t9006 + 0.11974241701863808564e0_f64 * t9009 - 0.8980681276397856423e-1_f64 * t9011 + 0.17961362552795712846e0_f64 * t9013 + 0.44903406381989282115e-1_f64 * t9015 - 0.8980681276397856423e-1_f64 * t9017 - 0.59871208509319042821e-1_f64 * t326 * t8866 - 0.2993560425465952141e-1_f64 * t9021 + 0.44903406381989282115e-1_f64 * t9023 + 0.59871208509319042821e-1_f64 * t305 * t9025;
    (t9025, t9028)
}
