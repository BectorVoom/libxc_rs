//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 815/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk815<F: Float>(t1646: F, t4681: F, t10568: F, t1417: F, t4660: F, t1797: F, t180: F, t479: F, t574: F, t682: F, t695: F, t10459: F, t707: F, t3521: F, t4616: F, t213: F, t568: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11361 = t1646 * t4681;
    let t11371 = 0.12841111111111111111e-1 * t10568;
    let t11390 = t1417 * t4660;
    let t11400 = t180 * t479 * t1797;
    let t11401 = t574 * t682;
    let t11402 = t11401 * t695;
    let t11417 = t10459 * t707;
    let t11423 = t3521 * t4616;
    let t11458 = t213 * t568;
    (t11361, t11371, t11390, t11400, t11401, t11402, t11417, t11423, t11458)
}
