//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1013/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1013<F: Float>(t2001: F, t6220: F, t1967: F, t9560: F, t6228: F, t6200: F, t9573: F, t17912: F, t2288: F, t31443: F, t8960: F, t8906: F, t35649: F, t8402: F, t15386: F, t1745: F, t2012: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t39840 = t2001 * t6220;
    let t39842 = t1967 * t9560;
    let t39844 = t2001 * t6228;
    let t39846 = t2001 * t6200;
    let t39848 = t1967 * t9573;
    let t39852 = t31443 * t17912 * t2288 * t8960;
    let t39854 = t2288 * t8906;
    let t39856 = t31443 * t35649 * t39854;
    let t39858 = t2288 * t8402;
    let t39860 = t31443 * t15386 * t39858;
    let t39862 = t2012 * t1745;
    (t39840, t39842, t39844, t39846, t39848, t39852, t39854, t39856, t39858, t39860, t39862)
}
