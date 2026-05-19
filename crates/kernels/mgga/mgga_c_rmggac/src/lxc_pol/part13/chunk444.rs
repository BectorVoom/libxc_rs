//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 444/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk444<F: Float>(t1310: F, t4709: F, t1295: F, t252: F, t4130: F, t4133: F, t4136: F, t4138: F, t240: F, t1302: F, t255: F, t1309: F, t16: F) -> (F, F, F, F, F) {
    let t4710 = t1310 * t4709;
    let t4712 = t1295 * t252;
    let t4719 = -F::cast_from(0.29633333333333333333e-1_f64) * t4130 + F::cast_from(0.19755555555555555555e-1_f64) * t4133 - F::cast_from(0.23048148148148148148e-1_f64) * t4136 - F::cast_from(0.32547666666666666667e-1_f64) * t4138;
    let t4720 = t240 * t4719;
    let t4724 = t255 * t1302;
    let t4728 = F::new(1.0) / t1309 / t16;
    (t4710, t4712, t4720, t4724, t4728)
}
