//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2303/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2303(t22892: f64, t22893: f64, t26388: f64, t7733: f64, t81186: f64, t5318: f64, t552: f64, t1307: f64, t6637: f64, t6888: f64, t1352: f64, t22633: f64, t6976: f64, t90754: f64) -> (f64, f64, f64, f64) {
    let t90805 = t22892 * t22893 * t26388;
    let t90806 = 0.16449340668482264365e-1_f64 * t90805;
    let t90807 = t81186 * t7733;
    let t90809 = t552 * t5318;
    let t90812 = t6888 * t6637 * t90809 * t1307;
    let t90816 = t22633 * t6976 * t90754 * t1352;
    (t90806, t90807, t90812, t90816)
}
