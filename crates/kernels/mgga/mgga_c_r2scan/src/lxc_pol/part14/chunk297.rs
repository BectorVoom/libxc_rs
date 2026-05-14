//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 297/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk297<F: Float>(t106: F, t797: F, t97: F, t986: F, t889: F, t298: F, t302: F, t308: F, t295: F, t305: F, t309: F, t814: F, t313: F, t825: F, rho1: F, tau1: F) -> (F, F, F, F, F, F, F) {
    let t989 = t97 * t106 * t986 * t797;
    let t990 = t889 / 2.0;
    let t991 = t298 * t990;
    let t994 = rho1 * rho1;
    let t996 = 1.0 / t302 / t994;
    let t997 = tau1 * t996;
    let t1000 = -t990;
    let t1001 = t308 * t1000;
    let t1004 = 5.0 / 3.0 * t295 * t991 - 5.0 / 3.0 * t997 * t309 + 5.0 / 3.0 * t305 * t1001 + t814;
    let t1010 = 3.0 / 10.0 * t313 * (5.0 / 3.0 * t991 + 5.0 / 3.0 * t1001) - t825;
    (t989, t990, t991, t997, t1000, t1004, t1010)
}
