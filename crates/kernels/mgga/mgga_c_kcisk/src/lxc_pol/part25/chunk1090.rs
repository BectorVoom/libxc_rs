//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1090/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1090<F: Float>(t32558: F, t32682: F, t233: F, t1065: F, t9406: F, t2707: F, t3299: F, t4573: F, t806: F, t2776: F, t1628: F, t2053: F, t5556: F, t566: F, t2819: F, t4574: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32683 = t32558 + t32682;
    let t32684 = t233 * t32683;
    let t32685 = t1065 * t9406;
    let t32693 = t3299 * t2707;
    let t32870 = t4573 * t806;
    let t32871 = t2776 * t32870;
    let t32872 = t32871 / 16.0;
    let t32873 = t1628 * t2053;
    let t32874 = t2776 * t32873;
    let t32875 = t32874 / 8.0;
    let t32876 = t566 * t5556;
    let t32877 = t2776 * t32876;
    let t32878 = t32877 / 16.0;
    let t32879 = t4574 * t2819;
    (t32683, t32684, t32685, t32693, t32870, t32872, t32873, t32875, t32876, t32878, t32879)
}
