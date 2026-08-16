//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1081/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1081(t41247: f64, t41257: f64, t36094: f64, t36096: f64, t36099: f64, t36101: f64, t36115: f64, t36117: f64, t41233: f64, t41235: f64, t41237: f64, t41239: f64, t41241: f64, t41243: f64, t41245: f64, t41255: f64) -> f64 {
    let t43558 = 0.77886770749688743854e-2_f64 * t41247;
    let t43566 = 0.2927036860455597649e0_f64 * t41257;
    let t43567 = -0.42483693136193860284e-2_f64 * t41233 + 0.39828462315181744017e-2_f64 * t41235 - 0.55759847241254441624e-2_f64 * t41237 + 0.39828462315181744017e-2_f64 * t41239 + 0.29738585195335702199e-1_f64 * t41241 - 0.53104616420242325356e-2_f64 * t41243 + 0.79656924630363488034e-2_f64 * t41245 + t43558 + 0.1333427903096438929e0_f64 * t36094 - 0.17779038707952519054e0_f64 * t36096 + 0.56448240417072397693e-3_f64 * t36099 - 0.18183107769496894486e-1_f64 * t36101 + 0.67737888500486877232e-2_f64 * t36115 + 0.14546486215597515589e0_f64 * t36117 + 0.79656924630363488034e-3_f64 * t41255 + t43566;
    t43567
}
