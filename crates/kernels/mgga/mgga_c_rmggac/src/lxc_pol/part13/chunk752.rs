//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 752/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk752<F: Float>(t31: F, t356: F, t290: F, t2012: F, t7349: F, t2019: F, t35214: F, t640: F, t7764: F, t7553: F, t7555: F, t1302: F, t131: F, t1310: F, t20: F, t2018: F, t2020: F, t252: F) -> (F, F, F, F, F) {
    let t35219 = t356 * t31;
    let t35220 = t290 * t35219;
    let t35222 = t7349 * t2012 * t35220;
    let t35226 = t2019 * t7764 * t640 * t35214;
    let t35228 = t640 * t35219;
    let t35230 = t7553 * t7555 * t35228;
    let t35238 = t1310 * t252 * t20 * t2018 * t2020 * t640 * t131 * t1302;
    (t35220, t35222, t35226, t35230, t35238)
}
