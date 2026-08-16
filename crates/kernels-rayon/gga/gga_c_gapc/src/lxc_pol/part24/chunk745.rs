//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 745/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk745(t1808: f64, t8858: f64, t1850: f64, t3039: f64, t122: f64, t1266: f64, t1034: f64, t1040: f64, t3061: f64, t3065: f64, t3060: f64, t3072: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8859 = t8858 * t1808;
    let t8861 = t3039 * t1850;
    let t8863 = t1266 * t122;
    let t8864 = t8863 * t1034;
    let t8865 = t8864 * t1040;
    let t8867 = t3061 * t3065;
    let t8869 = t3060 * t3072;
    (t8859, t8861, t8863, t8865, t8867, t8869)
}
