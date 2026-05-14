//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 841/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk841<F: Float>(t31404: F, t7507: F, t7517: F, t3088: F, t7646: F, t3453: F, t2138: F, t2147: F, t463: F, t7993: F, t7976: F, t880: F, t2155: F, t30005: F, t2122: F, t2132: F, t7885: F, t864: F) -> (F, F, F, F, F, F, F) {
    let t31867 = t7507 * t31404 * t7517;
    let t31868 = 0.1383716060742582691e-1 * t31867;
    let t31878 = t3088 * t7646;
    let t31879 = t31878 * t3453;
    let t31905 = t2138 * t2147 * t7993 * t463;
    let t31916 = t7976 * t880;
    let t31926 = t30005 * t2155;
    let t31955 = t7885 * t2132 * t2122 * t864;
    (t31868, t31878, t31879, t31905, t31916, t31926, t31955)
}
