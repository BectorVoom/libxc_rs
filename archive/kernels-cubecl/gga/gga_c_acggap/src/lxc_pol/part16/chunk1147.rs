//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1147/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1147<F: Float>(t31443: F, t35649: F, t39854: F, t2288: F, t8402: F, t15386: F, t1745: F, t2012: F, t31346: F, t5903: F, t35466: F, t6339: F) -> (F, F, F, F, F, F) {
    let t39856 = t31443 * t35649 * t39854;
    let t39858 = t2288 * t8402;
    let t39860 = t31443 * t15386 * t39858;
    let t39862 = t2012 * t1745;
    let t39867 = t31346 * t5903;
    let t39869 = t35466 * t6339;
    (t39856, t39858, t39860, t39862, t39867, t39869)
}
