//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2206/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2206(t26447: f64, t90607: f64, t90787: f64, t22751: f64, t26397: f64, t22892: f64, t22893: f64, t26396: f64, t26384: f64, t16018: f64, t6637: f64, t6888: f64, t6968: f64) -> (f64, f64, f64, f64, f64) {
    let t90789 = t90607 * t90787 * t26447;
    let t90791 = t22751 * t26397;
    let t90792 = 0.76763589786250567036e-1_f64 * t90791;
    let t90794 = t22892 * t22893 * t26396;
    let t90795 = 0.16449340668482264365e-1_f64 * t90794;
    let t90797 = t22892 * t22893 * t26384;
    let t90798 = 0.16449340668482264365e-1_f64 * t90797;
    let t90801 = t6888 * t6637 * t6968 * t16018;
    (t90789, t90792, t90795, t90798, t90801)
}
