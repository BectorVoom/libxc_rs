//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1121/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1121<F: Float>(t42201: F, t42204: F, t42206: F, t42217: F, t942: F, t9639: F, t1550: F, t2435: F, t26291: F, t26387: F, t36998: F, t37000: F, t38125: F, t42196: F, t42199: F, t42211: F, t42215: F, t42222: F, t44277: F, t5204: F, t5211: F, t530: F, t699: F, t903: F) -> F {
    let t44423 = F::cast_from(0.1454648621559751559e0_f64) * t42201;
    let t44424 = F::cast_from(0.35754263910370185096e-3_f64) * t42204;
    let t44425 = F::cast_from(0.23836175940246790064e-3_f64) * t42206;
    let t44428 = F::cast_from(0.11918087970123395032e-3_f64) * t42217;
    let t44431 = F::cast_from(0.4726e1_f64) * t942 * t9639;
    let t44440 = -F::cast_from(0.2363e1_f64) * t530 * t38125 - F::cast_from(0.71845450211182851384e0_f64) * t26291 * t44277 - F::cast_from(0.26668558061928778581e0_f64) * t42196 + F::cast_from(0.39914139006212695214e-1_f64) * t26387 * t2435 - F::cast_from(0.13637330827122670865e0_f64) * t42199 - t44423 + t44424 - t44425 + F::cast_from(0.5107751987195740728e-4_f64) * t42211 - F::cast_from(0.5107751987195740728e-4_f64) * t42215 + t44428 - F::cast_from(0.1702583995731913576e-4_f64) * t42222 - t44431 + F::cast_from(0.15965655602485078085e0_f64) * t36998 - F::cast_from(0.23948483403727617128e0_f64) * t1550 * t699 * t5204 + F::cast_from(0.35922725105591425692e0_f64) * t903 * t699 * t5211 - F::cast_from(0.79828278012425390427e-1_f64) * t37000;
    t44440
}
