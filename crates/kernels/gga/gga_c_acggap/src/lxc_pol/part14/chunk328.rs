//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 328/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk328<F: Float>(t336: F, t429: F, t513: F, t1140: F, t515: F, t1137: F, t500: F, t1050: F, t1063: F, t1124: F, t1126: F, t1130: F, t1474: F, t1477: F, t1481: F, t1484: F) -> (F, F, F, F) {
    let t1511 = t336 * t429 * t513;
    let t1514 = t1140 * t515;
    let t1516 = t1137 * t500;
    let t1524 = t1124 + F::new(0.489e0) * t1050 - t1126 + F::new(0.489e0) * t1474 + F::new(0.7335e0) * t1477 - F::new(0.61125e-1) * t1481 - F::new(0.36675e0) * t1484 - F::new(0.61125e-1) * t1063 + t1130;
    (t1511, t1514, t1516, t1524)
}
