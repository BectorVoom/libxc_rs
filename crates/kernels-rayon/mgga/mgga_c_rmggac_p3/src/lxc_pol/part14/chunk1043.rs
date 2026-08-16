//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1043/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1043(t1587: f64, t236: f64, t3351: f64, t498: f64, t7248: f64, t26157: f64, t5223: f64, t645: f64, t1635: f64, t2064: f64, t4044: f64, t1550: f64, t27102: f64, t7577: f64) -> (f64, f64, f64, f64) {
    let t41706 = t3351 * t7248 * t236 * t1587 * t498;
    let t41713 = t26157 * t645 * t5223;
    let t41716 = t4044 * t2064 * t1635;
    let t41717 = 0.95793933614910468512e0_f64 * t41716;
    let t41719 = t1550 * t7577 * t27102;
    (t41706, t41713, t41717, t41719)
}
