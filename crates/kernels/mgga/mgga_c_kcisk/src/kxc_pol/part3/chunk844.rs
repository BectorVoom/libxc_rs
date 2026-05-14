//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 844/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk844<F: Float>(t13803: F, t13849: F, t13889: F, t13932: F, t1328: F, t3579: F, t3764: F, t1340: F, t1339: F, t1440: F, t3732: F, t1341: F, t3785: F, t1411: F, t1286: F, t1450: F) -> (F, F, F, F, F, F) {
    let t13934 = t13803 + t13849 + t13889 + t13932;
    let t13935 = t13934 * t1328;
    let t13938 = t3764 * t3579;
    let t13939 = t1340 * t13938;
    let t13940 = t1339 * t13939;
    let t13944 = t3732 * t1440;
    let t13945 = t1341 * t13944;
    let t13946 = t3785 * t13945;
    let t13947 = t1411 * t13946;
    let t13949 = t3732 * t1286;
    let t13950 = t1450 * t13949;
    (t13935, t13940, t13944, t13947, t13949, t13950)
}
