//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 874/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk874<F: Float>(t874: F, t9486: F, t2447: F, t4616: F, t42023: F, t42026: F, t42044: F, t42086: F, t42101: F, t40803: F, t40831: F, t40907: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t43970 = t874 * t9486;
    let t43974 = t4616 * t2447;
    let t43978 = F::cast_from(0.162600798888400151e-2_f64) * t42023;
    let t43979 = F::cast_from(0.162600798888400151e-2_f64) * t42026;
    let t43987 = F::cast_from(0.11918087970123395032e-3_f64) * t42044;
    let t44004 = F::cast_from(0.39726959900411316772e-4_f64) * t42086;
    let t44008 = F::cast_from(0.11918087970123395032e-3_f64) * t42101;
    let t44029 = F::cast_from(0.3193131120497015617e0_f64) * t40803;
    let t44035 = F::cast_from(0.3193131120497015617e0_f64) * t40831;
    let t44070 = F::cast_from(0.21819729323396273384e0_f64) * t40907;
    (t43970, t43974, t43978, t43979, t43987, t44004, t44008, t44029, t44035, t44070)
}
