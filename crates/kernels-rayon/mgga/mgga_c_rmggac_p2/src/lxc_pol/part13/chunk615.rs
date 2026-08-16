//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 615/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk615(t7442: f64, t7445: f64, t7451: f64, t7458: f64, t7464: f64, t7470: f64, t7479: f64, t7485: f64, t7495: f64, t7499: f64, t7502: f64, t7506: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8096 = 0.5987120850931904282e-1_f64 * t7442;
    let t8097 = 0.8980681276397856423e-1_f64 * t7445;
    let t8098 = 0.1702583995731913576e-4_f64 * t7451;
    let t8099 = 0.212822999466489197e-4_f64 * t7458;
    let t8100 = 0.1702583995731913576e-4_f64 * t7464;
    let t8101 = 0.5107751987195740728e-4_f64 * t7470;
    let t8102 = 0.1702583995731913576e-4_f64 * t7479;
    let t8103 = 0.5107751987195740728e-4_f64 * t7485;
    let t8109 = 0.40911992481368012596e-1_f64 * t7495;
    let t8110 = 0.20455996240684006298e-1_f64 * t7499;
    let t8111 = 0.5454932330849068346e-1_f64 * t7502;
    let t8112 = 0.2727466165424534173e-1_f64 * t7506;
    (t8096, t8097, t8098, t8099, t8100, t8101, t8102, t8103, t8109, t8110, t8111, t8112)
}
