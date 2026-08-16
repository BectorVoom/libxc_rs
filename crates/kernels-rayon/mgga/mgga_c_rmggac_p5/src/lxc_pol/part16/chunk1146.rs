//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1146/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1146(t1356: f64, t1550: f64, t1624: f64, t1923: f64, t1953: f64, t2231: f64, t2471: f64, t38140: f64, t42132: f64, t44382: f64, t44385: f64, t47933: f64, t47935: f64, t47946: f64, t47948: f64, t47952: f64, t47957: f64, t47961: f64, t47963: f64, t49432: f64, t6344: f64, t702: f64, t72: f64, t8188: f64) -> f64 {
    let t49770 = -0.35922725105591425692e0_f64 * t47933 - 0.23948483403727617128e0_f64 * t47935 + 0.1454648621559751559e0_f64 * t42132 - t44382 - t44385 + 0.39914139006212695214e-1_f64 * t1356 * t49432 - 0.5107751987195740728e-4_f64 * t47946 - 0.5107751987195740728e-4_f64 * t47948 + 0.5107751987195740728e-4_f64 * t47952 - t38140 - 0.23948483403727617128e0_f64 * t1550 * t2471 * t1624 - 0.2363e1_f64 * t1923 * t8188 + 0.85129199786595678799e-5_f64 * t47957 - 0.2553875993597870364e-4_f64 * t47961 + 0.5107751987195740728e-4_f64 * t47963 + t72 * t1953 * t2231 + t72 * t6344 * t702;
    t49770
}
