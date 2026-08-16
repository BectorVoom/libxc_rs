//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1001/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1001(t78090: f64, t3839: f64, t71982: f64, t8632: f64, t69437: f64, t69445: f64, t25820: f64, t77091: f64, t27048: f64, t77338: f64, t76363: f64, t76365: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78091 = 0.40911992481368012592e-1_f64 * t78090;
    let t78093 = t3839 * t71982 * t8632;
    let t78094 = 0.6818665413561335432e-1_f64 * t78093;
    let t78098 = 0.21819729323396273382e0_f64 * t69437;
    let t78099 = 0.54549323308490683456e-1_f64 * t69445;
    let t78100 = t25820 * t77091;
    let t78101 = 0.8980681276397856423e-1_f64 * t78100;
    let t78103 = 0.35922725105591425692e0_f64 * t27048 * t77338;
    let t78110 = 0.10909864661698136691e0_f64 * t76363;
    let t78111 = 0.21819729323396273382e0_f64 * t76365;
    (t78091, t78094, t78098, t78099, t78101, t78103, t78110, t78111)
}
