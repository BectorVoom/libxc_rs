//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 586/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk586<F: Float>(t1713: F, t372: F, t1095: F, t1426: F, t175: F, t5645: F, t1008: F, t1861: F, t1089: F, t1859: F, t429: F, t1298: F, t506: F, t368: F, t1487: F, t495: F) -> (F, F, F, F, F, F, F, F) {
    let t5944 = t1713 * t372;
    let t5946 = t1426 * t1095 * t5944;
    let t5950 = t1426 * t175 * t5645;
    let t5953 = t1008 * t1861;
    let t5956 = t1089 * t429 * t1859;
    let t5959 = t1298 * t506;
    let t5961 = t1089 * t368 * t5959;
    let t5964 = t495 * t1487;
    (t5944, t5946, t5950, t5953, t5956, t5959, t5961, t5964)
}
