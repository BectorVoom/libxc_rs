//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 857/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk857<F: Float>(t10744: F, t2233: F, t3290: F, t120: F, t2222: F, t2225: F, t10700: F, t10702: F, t10705: F, t10713: F, t10714: F, t10717: F, t10720: F, t10723: F, t10726: F, t10730: F, t10732: F, t10742: F) -> (F,) {
    let t10745 = 0.25610080155860322884e0 * t10744;
    let t10746 = t3290 * t2233;
    let t10748 = t120 * t2222;
    let t10749 = t10748 * t2225;
    let t10751 = t10700 - 0.27439371595564631661e-1 * t10702 + 0.43663693315433241792e-2 * t10705 + t10713 - 0.86682217400542685632e-1 * t10714 + 0.54878743191129263322e-1 * t10717 + 0.86682217400542685632e-1 * t10720 + 0.2600466522016280569e0 * t10723 - 0.43341108700271342816e-1 * t10726 + 0.47609969197673950972e-2 * t10730 - 0.47609969197673950972e-2 * t10732 - t10742 + t10745 - 0.54878743191129263322e-1 * t10746 + 0.16463622957338778997e0 * t10749;
    (t10751,)
}
