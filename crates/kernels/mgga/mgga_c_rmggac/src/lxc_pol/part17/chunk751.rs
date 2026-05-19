//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 751/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk751<F: Float>(t35219: F, t640: F, t7553: F, t7555: F, t1302: F, t131: F, t1310: F, t20: F, t2018: F, t2020: F, t252: F, t2019: F, t2164: F, t7352: F, t7764: F) -> (F, F, F) {
    let t35228 = t640 * t35219;
    let t35230 = t7553 * t7555 * t35228;
    let t35238 = t1310 * t252 * t20 * t2018 * t2020 * t640 * t131 * t1302;
    let t35239 = F::cast_from(0.45731474687362542471e-3_f64) * t35238;
    let t35242 = t2019 * t7764 * t2164 * t7352;
    (t35230, t35239, t35242)
}
