//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 862/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk862(t146: f64, t147: f64, t9083: f64, t279: f64, t5123: f64, t5150: f64, t5179: f64, t5183: f64, t6062: f64, t7419: f64, t7459: f64, t7468: f64, t7472: f64, t7475: f64, t7479: f64, t7482: f64, t8861: f64, t8863: f64, t8867: f64, t8874: f64) -> (f64, f64) {
    let t9085 = t146 * t147 * t9083;
    let t9088 = 0.58544643236296698112e-1_f64 * t5123 + 0.81312004494856525156e-4_f64 * t5150 + 0.23115257973478049502e0_f64 * t8861 + 0.12805040077930161442e0_f64 * t8863 - 0.23115257973478049502e0_f64 * t8867 - 0.57829097596741960691e-3_f64 * t7419 + 0.679213007128961539e-1_f64 * t5179 + 0.2037639021386884617e0_f64 * t5183 - t7459 - t7468 + t7472 - 0.13869154784086829701e1_f64 * t8874 - t7475 - t7479 - t7482 + 0.43341108700271342816e-1_f64 * t9085 * t279 - t6062;
    (t9085, t9088)
}
