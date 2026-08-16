//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1817/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1817(t22591: f64, t7687: f64, t1983: f64, t1307: f64, t1845: f64, t8643: f64, t22574: f64, t15868: f64, t2019: f64, t1774: f64, t6534: f64, t652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25985 = t22591 * t7687;
    let t25987 = 3.0_f64 * t1983 * t25985;
    let t25988 = t1845 * t1307;
    let t25989 = t8643 * t25988;
    let t25991 = 3.0_f64 * t22574 * t25989;
    let t25992 = t2019 * t15868;
    let t25993 = t1983 * t25992;
    let t25994 = t1774 * t6534;
    let t25996 = 2.0_f64 * t652 * t25994;
    (t25985, t25987, t25988, t25989, t25991, t25992, t25993, t25994, t25996)
}
