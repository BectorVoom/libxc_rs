//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 894/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk894<F: Float>(t10834: F, t10842: F, t10853: F, t10812: F, t10815: F, t10821: F, t10824: F, t10827: F, t10829: F, t10837: F, t10839: F, t10846: F, t10850: F, t10857: F, t11417: F, t10863: F) -> (F, F, F, F, F) {
    let t11422 = 0.84755945902752848174e0 * t10834;
    let t11425 = 0.32927245914677557993e-1 * t10842;
    let t11428 = 0.16262400898971305031e-3 * t10853;
    let t11430 = 0.46230515946956099004e0 * t10812 - 0.86682217400542685632e-1 * t10815 - t11417 - 0.87327386630866483588e-2 * t10821 + 0.43663693315433241794e-2 * t10824 - 0.26198215989259945076e-1 * t10827 + 0.87327386630866483588e-2 * t10829 + t11422 + 0.43663693315433241794e-2 * t10837 - 0.46230515946956099004e0 * t10839 + t11425 - 0.93149212406257582492e-1 * t10846 - 0.27944763721877274748e0 * t10850 - t11428 - 0.19514881078765566037e-1 * t10857;
    let t11432 = 0.28914548798370980346e-3 * t10863;
    (t11422, t11425, t11428, t11430, t11432)
}
