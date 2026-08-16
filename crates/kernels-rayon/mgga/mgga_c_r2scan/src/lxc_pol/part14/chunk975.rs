//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 975/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk975(t10818: f64, t10834: f64, t10842: f64, t10853: f64, t10812: f64, t10815: f64, t10821: f64, t10824: f64, t10827: f64, t10829: f64, t10837: f64, t10839: f64, t10846: f64, t10850: f64, t10857: f64) -> (f64, f64, f64, f64, f64) {
    let t11417 = 0.58544643236296698113e-1_f64 * t10818;
    let t11422 = 0.84755945902752848174e0_f64 * t10834;
    let t11425 = 0.32927245914677557993e-1_f64 * t10842;
    let t11428 = 0.16262400898971305031e-3_f64 * t10853;
    let t11430 = 0.46230515946956099004e0_f64 * t10812 - 0.86682217400542685632e-1_f64 * t10815 - t11417 - 0.87327386630866483588e-2_f64 * t10821 + 0.43663693315433241794e-2_f64 * t10824 - 0.26198215989259945076e-1_f64 * t10827 + 0.87327386630866483588e-2_f64 * t10829 + t11422 + 0.43663693315433241794e-2_f64 * t10837 - 0.46230515946956099004e0_f64 * t10839 + t11425 - 0.93149212406257582492e-1_f64 * t10846 - 0.27944763721877274748e0_f64 * t10850 - t11428 - 0.19514881078765566037e-1_f64 * t10857;
    (t11417, t11422, t11425, t11428, t11430)
}
