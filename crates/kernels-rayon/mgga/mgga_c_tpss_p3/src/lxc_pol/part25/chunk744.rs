//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 744/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk744(t4923: f64, t904: f64, t2601: f64, t2608: f64, t3746: f64, t3795: f64, t4828: f64, t4832: f64, t4836: f64, t4848: f64, t4855: f64, t4861: f64, t4863: f64, t4867: f64, t4870: f64, t4873: f64) -> (f64, f64) {
    let t4924 = t4923 * t904;
    let t4939 = -0.1294625e1_f64 * t4848 + 0.258925e1_f64 * t4855 + t2601 + 0.20128333333333333334e0_f64 * t3746 - 0.20128333333333333333e0_f64 * t4828 + 0.60385e0_f64 * t4832 - 0.301925e0_f64 * t4836 + 0.82524375e-1_f64 * t4861 + 0.16504875e0_f64 * t4863 + t2608 + 0.11038e0_f64 * t3795 - 0.27595e-1_f64 * t4867 + 0.16557e0_f64 * t4870 - 0.82785e-1_f64 * t4873;
    (t4924, t4939)
}
