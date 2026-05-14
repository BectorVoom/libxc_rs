//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 901/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk901<F: Float>(t1214: F, t2258: F, t5296: F, t1042: F, t3617: F, t3363: F, t3172: F, t3590: F, t1247: F, t11231: F, t5302: F, t3612: F, t3610: F, t1263: F, t3584: F, t1122: F) -> (F, F, F, F, F, F) {
    let t12931 = t2258 * t1214;
    let t12932 = t5296 * t12931;
    let t12933 = t1042 * t12932;
    let t12936 = t3617 * t1214;
    let t12937 = t12936 * t3363;
    let t12938 = t1042 * t12937;
    let t12941 = t3172 * t3590;
    let t12942 = t1247 * t12941;
    let t12944 = t5302 * t11231;
    let t12945 = t1042 * t12944;
    let t12948 = t3172 * t3612;
    let t12949 = t3610 * t12948;
    let t12951 = t1263 * t3584;
    let t12952 = t12951 * t1122;
    (t12933, t12938, t12942, t12945, t12949, t12952)
}
