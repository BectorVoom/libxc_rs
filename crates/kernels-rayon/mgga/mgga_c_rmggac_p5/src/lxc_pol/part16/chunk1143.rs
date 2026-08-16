//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1143/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1143(t10387: f64, t1632: f64, t2471: f64, t42042: f64, t42057: f64, t43987: f64, t47800: f64, t47802: f64, t47804: f64, t47809: f64, t47814: f64, t47816: f64, t47821: f64, t47826: f64, t47831: f64, t47833: f64, t47835: f64, t49327: f64, t4985: f64, t5019: f64, t739: f64, t903: f64, t9302: f64) -> f64 {
    let t49709 = -0.23948483403727617128e0_f64 * t5019 * t10387 + 0.12195059916630011325e-2_f64 * t42042 - 0.40911992481368012596e-1_f64 * t47800 - 0.16364796992547205038e0_f64 * t47802 - 0.40911992481368012596e-1_f64 * t47804 - 0.3405167991463827152e-4_f64 * t47809 + 0.1702583995731913576e-4_f64 * t47814 + 0.212822999466489197e-4_f64 * t47816 + t43987 + 0.39726959900411316773e-4_f64 * t47821 + 0.212822999466489197e-4_f64 * t47826 + 0.17562221162733585894e1_f64 * t42057 - 0.11974241701863808564e0_f64 * t47831 - 0.14369090042236570277e1_f64 * t47833 + 0.35922725105591425692e0_f64 * t903 * t2471 * t1632 + 0.31931311204970156171e0_f64 * t47835 - 0.59871208509319042821e-1_f64 * t739 * t49327 + 0.11974241701863808564e0_f64 * t4985 * t9302;
    t49709
}
