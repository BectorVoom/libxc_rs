//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 369/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk369(t2181: f64, t2193: f64, t2197: f64, t2185: f64, t2187: f64) -> (f64, f64) {
    let t2201 = -0.34752604166666666667e-3_f64 * t2193 * t2197 + 0.17411041666666666666e-2_f64 * t2181;
    let t2205 = 0.9375e-1_f64 * t2185 - 0.20234375e-1_f64 * t2187;
    (t2201, t2205)
}
