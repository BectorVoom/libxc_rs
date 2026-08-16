//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1138/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1138(t125: f64, t8536: f64, t8538: f64, t86: f64, t2421: f64, t7603: f64, t137: f64, t8963: f64, t26446: f64, t710: f64, t8999: f64, t754: f64, t8750: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91913 = t86 * t125 * t8536 * t8538;
    let t91916 = t86 * t2421 * t7603;
    let t91919 = t86 * t8963 * t137;
    let t91922 = t86 * t710 * t26446;
    let t91925 = t86 * t8999 * t137;
    let t91929 = t86 * t125 * t754 * t8750;
    (t91913, t91916, t91919, t91922, t91925, t91929)
}
