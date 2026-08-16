//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1002/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1002(t40734: f64, t5259: f64, t321: f64, t333: f64, t35873: f64, t35877: f64, t35886: f64, t35890: f64, t41015: f64, t41077: f64, t41079: f64, t41084: f64, t41086: f64, t41088: f64, t41091: f64, t4669: f64, t5148: f64, t838: f64) -> f64 {
    let t41095 = t5259 * t40734;
    let t41097 = 0.27274661654245341729e-1_f64 * t35873 - 0.20001418546446583934e0_f64 * t35877 + 0.18183107769496894486e0_f64 * t35886 + 0.54549323308490683458e-1_f64 * t35890 - 0.8980681276397856423e0_f64 * t41077 + 0.5987120850931904282e-1_f64 * t41079 - 0.23948483403727617128e0_f64 * t5148 * t41015 * t321 - 0.17961362552795712846e0_f64 * t41084 - 0.5987120850931904282e-1_f64 * t41086 + 0.23948483403727617128e0_f64 * t838 * t41088 - 0.35922725105591425692e0_f64 * t4669 * t41091 * t333 - 0.5987120850931904282e-1_f64 * t41095;
    t41097
}
