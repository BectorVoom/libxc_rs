//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 934/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk934<F: Float>(t10740: F, t254: F, t120: F, t2176: F, t531: F, t2233: F, t3290: F, t2222: F, t2225: F, t10700: F, t10702: F, t10705: F, t10713: F, t10714: F, t10717: F, t10720: F, t10723: F, t10726: F, t10730: F, t10732: F) -> (F, F, F, F) {
    let t10741 = t254 * t10740;
    let t10742 = F::cast_from(0.15573871527278325618e-1_f64) * t10741;
    let t10743 = t120 * t2176;
    let t10744 = t10743 * t531;
    let t10745 = F::cast_from(0.25610080155860322884e0_f64) * t10744;
    let t10746 = t3290 * t2233;
    let t10748 = t120 * t2222;
    let t10749 = t10748 * t2225;
    let t10751 = t10700 - F::cast_from(0.27439371595564631661e-1_f64) * t10702 + F::cast_from(0.43663693315433241792e-2_f64) * t10705 + t10713 - F::cast_from(0.86682217400542685632e-1_f64) * t10714 + F::cast_from(0.54878743191129263322e-1_f64) * t10717 + F::cast_from(0.86682217400542685632e-1_f64) * t10720 + F::cast_from(0.2600466522016280569e0_f64) * t10723 - F::cast_from(0.43341108700271342816e-1_f64) * t10726 + F::cast_from(0.47609969197673950972e-2_f64) * t10730 - F::cast_from(0.47609969197673950972e-2_f64) * t10732 - t10742 + t10745 - F::cast_from(0.54878743191129263322e-1_f64) * t10746 + F::cast_from(0.16463622957338778997e0_f64) * t10749;
    (t10742, t10743, t10744, t10751)
}
