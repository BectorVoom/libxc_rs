//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 976/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk976<F: Float>(t10863: F, t10866: F, t10901: F, t10860: F, t10870: F, t10873: F, t10876: F, t10880: F, t10883: F, t10886: F, t10889: F, t10892: F, t10895: F, t10897: F, t10905: F, t10909: F) -> (F, F, F, F) {
    let t11432 = F::new(0.28914548798370980346e-3) * t10863;
    let t11433 = F::new(0.42683466926433871473e0) * t10866;
    let t11444 = F::new(0.45022119329691164871e0) * t10901;
    let t11447 = F::new(0.86682217400542685632e-1) * t10860 + t11432 + t11433 - F::new(0.93149212406257582492e-1) * t10870 - F::new(0.17336443480108537126e0) * t10873 - F::new(0.86682217400542685632e-1) * t10876 - F::new(0.5200933044032561138e0) * t10880 - F::new(0.2600466522016280569e0) * t10883 + F::new(0.46230515946956099004e0) * t10886 + F::new(0.10401866088065122276e1) * t10889 + F::new(0.13869154784086829701e1) * t10892 + F::new(0.10975748638225852664e-1) * t10895 - F::new(0.39029762157531132074e-1) * t10897 - t11444 + F::new(0.93149212406257582492e-1) * t10905 - F::new(0.43663693315433241794e-2) * t10909;
    (t11432, t11433, t11444, t11447)
}
