//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2365/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2365(t100908: f64, t100915: f64, t100917: f64, t100921: f64, t100924: f64, t100927: f64, t100929: f64, t100932: f64, t100934: f64, t100936: f64, t100938: f64, t100941: f64, t1458: f64, t20176: f64, t20181: f64, t24972: f64, t27921: f64, t4072: f64, t5376: f64, t5456: f64, t85416: f64, t96311: f64, t96334: f64) -> f64 {
    let t105128 = 27.0_f64 * t24972 * t20181 + t100908 + 27.0_f64 * t96311 * t1458 + t100915 + 27.0_f64 * t85416 * t5456 + t100917 + t100921 + t100924 + t100927 + t100929 + t100932 + 54.0_f64 * t96334 * t5376 + 27.0_f64 * t27921 * t4072 + 54.0_f64 * t24972 * t20176 + t100934 + t100936 + t100938 + t100941;
    t105128
}
