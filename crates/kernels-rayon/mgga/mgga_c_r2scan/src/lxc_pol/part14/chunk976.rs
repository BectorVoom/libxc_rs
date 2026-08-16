//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 976/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk976(t10863: f64, t10866: f64, t10901: f64, t10860: f64, t10870: f64, t10873: f64, t10876: f64, t10880: f64, t10883: f64, t10886: f64, t10889: f64, t10892: f64, t10895: f64, t10897: f64, t10905: f64, t10909: f64) -> (f64, f64, f64, f64) {
    let t11432 = 0.28914548798370980346e-3_f64 * t10863;
    let t11433 = 0.42683466926433871473e0_f64 * t10866;
    let t11444 = 0.45022119329691164871e0_f64 * t10901;
    let t11447 = 0.86682217400542685632e-1_f64 * t10860 + t11432 + t11433 - 0.93149212406257582492e-1_f64 * t10870 - 0.17336443480108537126e0_f64 * t10873 - 0.86682217400542685632e-1_f64 * t10876 - 0.5200933044032561138e0_f64 * t10880 - 0.2600466522016280569e0_f64 * t10883 + 0.46230515946956099004e0_f64 * t10886 + 0.10401866088065122276e1_f64 * t10889 + 0.13869154784086829701e1_f64 * t10892 + 0.10975748638225852664e-1_f64 * t10895 - 0.39029762157531132074e-1_f64 * t10897 - t11444 + 0.93149212406257582492e-1_f64 * t10905 - 0.43663693315433241794e-2_f64 * t10909;
    (t11432, t11433, t11444, t11447)
}
