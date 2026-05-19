//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1081/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1081<F: Float>(t41247: F, t41257: F, t36094: F, t36096: F, t36099: F, t36101: F, t36115: F, t36117: F, t41233: F, t41235: F, t41237: F, t41239: F, t41241: F, t41243: F, t41245: F, t41255: F) -> F {
    let t43558 = F::cast_from(0.77886770749688743854e-2_f64) * t41247;
    let t43566 = F::cast_from(0.2927036860455597649e0_f64) * t41257;
    let t43567 = -F::cast_from(0.42483693136193860284e-2_f64) * t41233 + F::cast_from(0.39828462315181744017e-2_f64) * t41235 - F::cast_from(0.55759847241254441624e-2_f64) * t41237 + F::cast_from(0.39828462315181744017e-2_f64) * t41239 + F::cast_from(0.29738585195335702199e-1_f64) * t41241 - F::cast_from(0.53104616420242325356e-2_f64) * t41243 + F::cast_from(0.79656924630363488034e-2_f64) * t41245 + t43558 + F::cast_from(0.1333427903096438929e0_f64) * t36094 - F::cast_from(0.17779038707952519054e0_f64) * t36096 + F::cast_from(0.56448240417072397693e-3_f64) * t36099 - F::cast_from(0.18183107769496894486e-1_f64) * t36101 + F::cast_from(0.67737888500486877232e-2_f64) * t36115 + F::cast_from(0.14546486215597515589e0_f64) * t36117 + F::cast_from(0.79656924630363488034e-3_f64) * t41255 + t43566;
    t43567
}
