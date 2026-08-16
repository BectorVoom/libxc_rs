//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1241/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1241(t11968: f64, t1873: f64, t652: f64, t2314: f64, t23831: f64, t22607: f64, t7000: f64, t6880: f64, t22592: f64, t6876: f64, t22949: f64, t12020: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t80625 = 2.0_f64 * t652 * t11968 * t1873;
    let t80627 = 6.0_f64 * t2314 * t23831;
    let t80629 = 3.0_f64 * t22607 * t7000;
    let t80633 = 9.0_f64 * t22607 * t6880;
    let t80635 = 18.0_f64 * t6876 * t22592;
    let t80637 = 3.0_f64 * t6876 * t22949;
    let t80640 = t225 * t12020;
    (t80625, t80627, t80629, t80633, t80635, t80637, t80640)
}
