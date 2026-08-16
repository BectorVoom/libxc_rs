//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 376/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk376(t2245: f64, t2257: f64, t2260: f64, t2249: f64, t2251: f64) -> (f64, f64) {
    let t2264 = -0.34752604166666666667e-3_f64 * t2257 * t2260 + 0.17411041666666666666e-2_f64 * t2245;
    let t2268 = 0.9375e-1_f64 * t2249 - 0.20234375e-1_f64 * t2251;
    (t2264, t2268)
}
