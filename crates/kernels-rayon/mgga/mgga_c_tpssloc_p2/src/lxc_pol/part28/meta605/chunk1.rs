//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1912/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1912(t16044: f64, t1992: f64, t6976: f64, t1372: f64, t1799: f64, t1307: f64, t26331: f64, t26446: f64, t26411: f64, t6914: f64, t12420: f64, t5335: f64) -> (f64, f64, f64, f64, f64) {
    let t90752 = t1992 * t6976 * t16044;
    let t90754 = t1372 * t1799;
    let t90757 = t26331 * t26446 * t90754 * t1307;
    let t90759 = t6914 * t26411;
    let t90763 = t26331 * t6976 * t5335 * t12420;
    (t90752, t90754, t90757, t90759, t90763)
}
