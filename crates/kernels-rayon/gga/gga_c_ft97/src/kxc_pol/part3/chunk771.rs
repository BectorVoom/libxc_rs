//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 771/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk771(t15931: f64, t15966: f64, t348: f64, t1882: f64, t4603: f64, t4599: f64, t3291: f64, t447: f64, t925: f64, t3052: f64, t986: f64, t379: f64, t4623: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15967 = t15931 + t15966;
    let t15968 = t348 * t15967;
    let t15978 = t1882 * t4603;
    let t15980 = t1882 * t4599;
    let t15983 = t447 * t3291 * t925;
    let t15987 = t447 * t986 * t3052;
    let t15991 = t447 * t4623 * t379;
    (t15968, t15978, t15980, t15983, t15987, t15991)
}
