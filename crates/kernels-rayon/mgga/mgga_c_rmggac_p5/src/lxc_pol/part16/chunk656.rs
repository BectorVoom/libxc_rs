//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 656/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk656(t739: f64, t9437: f64, t8710: f64, t8716: f64, t8718: f64, t8125: f64, t8702: f64, t8706: f64, t8714: f64, t8720: f64, t8722: f64, t8724: f64, t8726: f64) -> (f64, f64) {
    let t9438 = t739 * t9437;
    let t9445 = 0.4838420607177634088e-3_f64 * t8710;
    let t9447 = 0.18183107769496894486e-1_f64 * t8716;
    let t9448 = 0.24244143692662525982e-1_f64 * t8718;
    let t9453 = -0.90915538847484472432e-2_f64 * t8702 + 0.1814407727691612783e-3_f64 * t8706 - t9445 + 0.56448240417072397693e-3_f64 * t8714 - t9447 + t9448 - 0.21168090156402149135e-3_f64 * t8720 + 0.68186654135613354324e-2_f64 * t8722 + 0.39828462315181744017e-2_f64 * t8724 - 0.55759847241254441624e-2_f64 * t8726 + t8125;
    (t9438, t9453)
}
