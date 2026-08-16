//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 388/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk388(t60: f64, t2398: f64, t63: f64, t697: f64, t72: f64, t700: f64, t209: f64, t2379: f64) -> (f64, f64, f64, f64, f64) {
    let t70 = 0.0_f64 < t60;
    let t2399 = t63 * t2398;
    let t2403 = 1.0_f64 / t697 / t72;
    let t2404 = t700 * t700;
    let t2406 = t209 * t2403 * t2404;
    let t2410 = piecewise3(t70, t2379, -t2379);
    (t2399, t2403, t2404, t2406, t2410)
}
