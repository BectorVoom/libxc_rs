//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1402/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1402(t21125: f64, t6151: f64, t18183: f64, t21130: f64, t18187: f64, t21134: f64, t1595: f64, t7403: f64, t12842: f64, t18093: f64, t18192: f64, t23115: f64, t23119: f64, t23123: f64, t23126: f64, t23130: f64, t4439: f64, t6152: f64, t6156: f64, t6160: f64) -> f64 {
    let t23133 = t6151 * t21125;
    let t23136 = t18183 * t21130;
    let t23139 = t18187 * t21134;
    let t23149 = t7403 * t1595;
    let t23151 = -t4439 * t23115 / 288.0_f64 + t4439 * t23119 / 144.0_f64 + t4439 * t23123 / 288.0_f64 + t4439 * t23126 / 96.0_f64 - t4439 * t23130 / 432.0_f64 - t4439 * t23133 / 72.0_f64 + 7.0_f64 / 1296.0_f64 * t4439 * t23136 - t4439 * t23139 / 108.0_f64 + t18192 * t6156 / 108.0_f64 + t18192 * t6160 / 54.0_f64 - t18192 * t6152 / 81.0_f64 + t12842 / 864.0_f64 - t18093 + 11.0_f64 / 648.0_f64 * t23149;
    t23151
}
