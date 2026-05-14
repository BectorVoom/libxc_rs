//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 872/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk872<F: Float>(t10990: F, t10995: F, t10741: F, t10758: F, t10818: F, t10834: F, t10842: F, t10853: F, t10863: F, t10866: F, t10901: F, t11017: F, t11002: F, t1115: F, t792: F, t2867: F, t481: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11379 = 0.29810146462873361016e-2 * t10990;
    let t11380 = 0.60975299583150056624e-3 * t10995;
    let t11393 = 0.31147743054556651237e-1 * t10741;
    let t11399 = 0.11902492299418487743e0 * t10758;
    let t11417 = 0.58544643236296698113e-1 * t10818;
    let t11422 = 0.84755945902752848174e0 * t10834;
    let t11425 = 0.32927245914677557993e-1 * t10842;
    let t11428 = 0.16262400898971305031e-3 * t10853;
    let t11432 = 0.28914548798370980346e-3 * t10863;
    let t11433 = 0.42683466926433871473e0 * t10866;
    let t11444 = 0.45022119329691164871e0 * t10901;
    let t11454 = 0.39032073591371545778e-3 * t11017;
    let t11465 = t11002 * t1115 * t792;
    let t11475 = t2867 * t481;
    (t11379, t11380, t11393, t11399, t11417, t11422, t11425, t11428, t11432, t11433, t11444, t11454, t11465, t11475)
}
