//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 740/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk740(t2515: f64, t4826: f64, t141: f64, t4830: f64, t861: f64, t4834: f64, t2499: f64, t2512: f64, t3746: f64, t3795: f64, t4828: f64, t4832: f64, t4836: f64, t4848: f64, t4855: f64, t4861: f64, t4863: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4866 = t2515 * t4826;
    let t4867 = t141 * t4866;
    let t4869 = t861 * t4830;
    let t4870 = t141 * t4869;
    let t4872 = t861 * t4834;
    let t4873 = t141 * t4872;
    let t4875 = -0.9494625e0_f64 * t4848 + 0.1898925e1_f64 * t4855 + t2499 + 0.19931111111111111111e0_f64 * t3746 - 0.19931111111111111111e0_f64 * t4828 + 0.59793333333333333334e0_f64 * t4832 - 0.29896666666666666667e0_f64 * t4836 + 0.15358125e0_f64 * t4861 + 0.3071625e0_f64 * t4863 + t2512 + 0.10954222222222222222e0_f64 * t3795 - 0.27385555555555555556e-1_f64 * t4867 + 0.16431333333333333333e0_f64 * t4870 - 0.82156666666666666667e-1_f64 * t4873;
    (t4866, t4867, t4869, t4870, t4872, t4873, t4875)
}
