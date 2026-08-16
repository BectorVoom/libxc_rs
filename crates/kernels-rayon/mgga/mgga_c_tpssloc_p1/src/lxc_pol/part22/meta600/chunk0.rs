//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2122/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2122(t1580: f64, t2930: f64, t2885: f64, t4408: f64, t47705: f64, t47707: f64, t47730: f64, t10632: f64, t4471: f64, t48096: f64, t2904: f64, t4446: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48783 = t2930 * t1580;
    let t48789 = t4408 * t2885;
    let t48799 = 0.4566222222222222222e-1_f64 * t47705;
    let t48800 = 0.1522074074074074074e-1_f64 * t47707;
    let t48809 = 0.2283111111111111111e-1_f64 * t47730;
    let t48890 = t4471 * t10632;
    let t48919 = 0.27385555555555555556e0_f64 * t48096;
    let t48924 = 0.39862222222222222223e0_f64 * t47730;
    let t48946 = 8.0_f64 / 9.0_f64 * t47705;
    let t48947 = 8.0_f64 / 27.0_f64 * t47707;
    let t48956 = 4.0_f64 / 9.0_f64 * t47730;
    let t49096 = t4446 * t2904;
    (t48783, t48789, t48799, t48800, t48809, t48890, t48919, t48924, t48946, t48947, t48956, t49096)
}
