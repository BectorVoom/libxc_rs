//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 766/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk766(t1527: f64, t2788: f64, t4983: f64, t2461: f64, t879: f64, t2321: f64, t955: f64, t6897: f64, t986: f64, t5021: f64, t5872: f64, t5874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6959 = t2788 * t1527;
    let t6961 = 48.0_f64 * t4983;
    let t6963 = 2.0_f64 * t879 * t2461;
    let t6966 = t2321 * t955;
    let t6967 = t986 * t6897;
    let t7025 = 4.0_f64 * t5021;
    let t7026 = 1584.0_f64 * t5872;
    let t7027 = 1872.0_f64 * t5874;
    (t6959, t6961, t6963, t6966, t6967, t7025, t7026, t7027)
}
