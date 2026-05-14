//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 683/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk683<F: Float>(t11450: F, t1801: F, t1873: F, t1869: F, t10375: F, t1900: F, t213: F, t568: F, t682: F, t1810: F, t1846: F, t1825: F, t5082: F, t5097: F, t696: F, t1806: F, t5105: F) -> (F, F, F, F, F, F, F, F) {
    let t11451 = t1801 * t11450;
    let t11452 = t1873 * t11451;
    let t11453 = t1869 * t11452;
    let t11455 = t10375 * t1900;
    let t11456 = t1869 * t11455;
    let t11458 = t213 * t568;
    let t11460 = 0.14055920378328537299e-1 * t11458 * t682;
    let t11461 = t1846 * t1810;
    let t11463 = t5082 * t1825;
    let t11465 = t696 * t5097;
    let t11467 = t1806 * t5105;
    (t11453, t11456, t11458, t11460, t11461, t11463, t11465, t11467)
}
