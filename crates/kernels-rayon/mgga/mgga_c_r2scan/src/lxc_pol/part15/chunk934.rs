//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 934/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk934(t10740: f64, t254: f64, t120: f64, t2176: f64, t531: f64, t2233: f64, t3290: f64, t2222: f64, t2225: f64, t10700: f64, t10702: f64, t10705: f64, t10713: f64, t10714: f64, t10717: f64, t10720: f64, t10723: f64, t10726: f64, t10730: f64, t10732: f64) -> (f64, f64, f64, f64) {
    let t10741 = t254 * t10740;
    let t10742 = 0.15573871527278325618e-1_f64 * t10741;
    let t10743 = t120 * t2176;
    let t10744 = t10743 * t531;
    let t10745 = 0.25610080155860322884e0_f64 * t10744;
    let t10746 = t3290 * t2233;
    let t10748 = t120 * t2222;
    let t10749 = t10748 * t2225;
    let t10751 = t10700 - 0.27439371595564631661e-1_f64 * t10702 + 0.43663693315433241792e-2_f64 * t10705 + t10713 - 0.86682217400542685632e-1_f64 * t10714 + 0.54878743191129263322e-1_f64 * t10717 + 0.86682217400542685632e-1_f64 * t10720 + 0.2600466522016280569e0_f64 * t10723 - 0.43341108700271342816e-1_f64 * t10726 + 0.47609969197673950972e-2_f64 * t10730 - 0.47609969197673950972e-2_f64 * t10732 - t10742 + t10745 - 0.54878743191129263322e-1_f64 * t10746 + 0.16463622957338778997e0_f64 * t10749;
    (t10742, t10743, t10744, t10751)
}
