//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1257/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1257<F: Float>(t35532: F, t567: F, t564: F, t565: F, t8463: F, t2777: F, t10043: F, t2360: F, t806: F, t8471: F, t2776: F, t2359: F, t2670: F, t2356: F, t9915: F, t10044: F) -> (F, F, F, F, F, F, F, F) {
    let t35533 = t567 * t35532;
    let t35534 = t564 * t35533;
    let t35535 = t35534 / 16.0;
    let t35536 = t8463 * t565;
    let t35537 = t35536 * t2777;
    let t35538 = t35537 / 8.0;
    let t35539 = t2360 * t10043;
    let t35540 = t564 * t35539;
    let t35541 = t35540 / 8.0;
    let t35542 = t8471 * t806;
    let t35543 = t2776 * t35542;
    let t35544 = t35543 / 16.0;
    let t35545 = t2359 * t2670;
    let t35546 = t2776 * t35545;
    let t35547 = t35546 / 8.0;
    let t35549 = t2356 * t9915;
    let t35550 = t35549 / 8.0;
    let t35551 = t2356 * t10044;
    (t35535, t35536, t35538, t35541, t35544, t35547, t35550, t35551)
}
