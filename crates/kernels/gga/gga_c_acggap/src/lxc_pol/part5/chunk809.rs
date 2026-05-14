//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 809/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk809<F: Float>(t158: F, t2933: F, t944: F, t3045: F, t3055: F, t1210: F, t939: F, t3084: F, t322: F, t113: F, t11805: F, t11820: F, t4: F, t381: F, t452: F, t1258: F, t980: F) -> (F, F, F, F, F, F, F, F) {
    let t12331 = 1.0 / t2933 / t158;
    let t12334 = t944 * t944;
    let t12344 = 0.15805078039045227836e2 * t3055 * t3045;
    let t12345 = t939 * t1210;
    let t12349 = t3084 * t322;
    let t12357 = 0.43209876543209876543e0 * t4 * t11805 * t113 + 0.27437962962962962965e0 * t11820;
    let t12360 = 0.65854491829355115987e0 * t381 * t452 * t12357;
    let t12385 = t980 * t1258;
    (t12331, t12334, t12344, t12345, t12349, t12357, t12360, t12385)
}
