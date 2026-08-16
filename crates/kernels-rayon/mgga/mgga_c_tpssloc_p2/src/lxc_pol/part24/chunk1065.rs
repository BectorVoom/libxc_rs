//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1065/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1065(t535: f64, t9534: f64, t9538: f64, t12188: f64, t12190: f64, t12194: f64, t12196: f64, t12197: f64, t12200: f64, t12205: f64, t12209: f64, t12212: f64, t12215: f64, t12217: f64, t12222: f64, t12228: f64, t12231: f64, t1315: f64, t5195: f64) -> f64 {
    let t12236 = 0.13888888888888888889e-3_f64 * t9534 * t535 * t9538;
    let t12237 = -t12188 - 0.38888888888888888888e-1_f64 * t12190 - t12194 + t12196 + 0.11666666666666666666e-1_f64 * t12197 - 0.15833333333333333333e-1_f64 * t12200 - 0.74999999999999999997e-2_f64 * t12205 + 0.24999999999999999999e-2_f64 * t12209 - 0.34999999999999999998e-1_f64 * t12212 - 0.19999999999999999999e-1_f64 * t12215 * t12217 + 0.14999999999999999999e-1_f64 * t5195 * t12222 + 0.49999999999999999998e-2_f64 * t12228 - 0.16666666666666666666e-2_f64 * t1315 * t12231 - t12236;
    t12237
}
