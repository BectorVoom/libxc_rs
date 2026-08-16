//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 575/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk575(t14259: f64, t14303: f64, t14306: f64, t14312: f64, t14431: f64, t14432: f64, t14433: f64, t14440: f64, t14443: f64, t14447: f64, t14450: f64, t14454: f64, t14457: f64, t14460: f64, t14461: f64, t14462: f64, t14463: f64, t14464: f64, t14468: f64, t14471: f64, t14500: f64) -> (f64, f64) {
    let t14996 = 0.58171619854173713844e-5_f64 * t14259;
    let t15000 = t14431 - t14432 - t14433 - t14440 - t14443 + t14447 - t14450 - t14454 + t14457 + t14460 - t14461 + t14462 - t14463 - t14464 - 0.93188427318671584242e-2_f64 * t14303 + 0.15531404553111930707e-1_f64 * t14306 + 0.31062809106223861414e-2_f64 * t14312 + t14468 + t14471 - t14500;
    (t14996, t15000)
}
