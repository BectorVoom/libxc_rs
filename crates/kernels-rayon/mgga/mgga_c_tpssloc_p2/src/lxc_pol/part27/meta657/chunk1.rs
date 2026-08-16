//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2298/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2298(t81311: f64, t16065: f64, t1992: f64, t22897: f64, t26378: f64, t6914: f64, t16044: f64, t6976: f64, t1372: f64, t1799: f64, t1307: f64, t26331: f64, t26446: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90743 = 0.16449340668482264365e-1_f64 * t81311;
    let t90747 = t1992 * t22897 * t16065;
    let t90749 = t6914 * t26378;
    let t90750 = 0.76763589786250567036e-1_f64 * t90749;
    let t90752 = t1992 * t6976 * t16044;
    let t90754 = t1372 * t1799;
    let t90757 = t26331 * t26446 * t90754 * t1307;
    (t90743, t90747, t90750, t90752, t90754, t90757)
}
