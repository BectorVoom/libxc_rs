//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 739/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk739(t2481: f64, t4844: f64, t1415: f64, t2487: f64, t2491: f64, t3746: f64, t4828: f64, t4832: f64, t4836: f64, t847: f64, t2504: f64, t854: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4846 = 2.0_f64 * t2481 * t4844;
    let t4847 = t1415 * t1415;
    let t4848 = t2487 * t4847;
    let t4854 = t2491 + 2.0_f64 / 9.0_f64 * t3746 - 2.0_f64 / 9.0_f64 * t4828 + 2.0_f64 / 3.0_f64 * t4832 - t4836 / 3.0_f64;
    let t4855 = t847 * t4854;
    let t4861 = t2504 * t4847;
    let t4863 = t854 * t4854;
    (t4846, t4847, t4848, t4854, t4855, t4861, t4863)
}
