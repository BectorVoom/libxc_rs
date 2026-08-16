//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2162/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2162(t54047: f64, t40167: f64, t820: f64, t16060: f64, t3798: f64, t12345: f64, t5310: f64, t1827: f64, t40123: f64, t3802: f64, t39947: f64, t1788: f64, t9212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54048 = 119.0_f64 / 4608.0_f64 * t54047;
    let t54063 = t40167 * t820;
    let t54124 = t16060 * t3798;
    let t54131 = t12345 * t5310;
    let t54132 = 595.0_f64 / 1152.0_f64 * t54131;
    let t54151 = t40123 * t1827;
    let t54162 = t16060 * t3802;
    let t54198 = t39947 * t1827;
    let t54199 = 119.0_f64 / 4608.0_f64 * t54198;
    let t54312 = t9212 * t1788;
    (t54048, t54063, t54124, t54132, t54151, t54162, t54199, t54312)
}
