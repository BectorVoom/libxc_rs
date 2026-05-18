//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 955/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk955<F: Float>(t10964: F, t10969: F, t10973: F, t10982: F, t10990: F, t10995: F, t10741: F, t10758: F, t10818: F, t10834: F, t10842: F, t10853: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11374 = F::new(0.30487649791575028312e-3) * t10964;
    let t11375 = F::new(0.68400385060046895e-6) * t10969;
    let t11377 = F::new(0.60975299583150056624e-3) * t10973;
    let t11378 = F::new(0.86737941314158990616e-4) * t10982;
    let t11379 = F::new(0.29810146462873361016e-2) * t10990;
    let t11380 = F::new(0.60975299583150056624e-3) * t10995;
    let t11393 = F::new(0.31147743054556651237e-1) * t10741;
    let t11399 = F::new(0.11902492299418487743e0) * t10758;
    let t11417 = F::new(0.58544643236296698113e-1) * t10818;
    let t11422 = F::new(0.84755945902752848174e0) * t10834;
    let t11425 = F::new(0.32927245914677557993e-1) * t10842;
    let t11428 = F::new(0.16262400898971305031e-3) * t10853;
    (t11374, t11375, t11377, t11378, t11379, t11380, t11393, t11399, t11417, t11422, t11425, t11428)
}
