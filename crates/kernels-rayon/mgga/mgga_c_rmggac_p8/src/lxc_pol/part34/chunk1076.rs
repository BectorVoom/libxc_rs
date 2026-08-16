//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1076/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1076(t78468: f64, t15421: f64, t4965: f64, t70320: f64, t71775: f64, t739: f64, t76036: f64, t78084: f64, t78438: f64, t78439: f64, t78440: f64, t78444: f64, t78446: f64, t78451: f64, t78454: f64, t78457: f64, t78462: f64, t78464: f64, t78465: f64) -> f64 {
    let t78469 = 0.15243824895787514157e-3_f64 * t78468;
    let t78470 = t78438 - t78439 + t78440 + 0.39914139006212695214e-1_f64 * t4965 * t15421 + t78444 + t78446 - t78451 - 0.17519306092901367187e-5_f64 * t76036 + t78454 - t78457 - t71775 - 0.59871208509319042821e-1_f64 * t739 * t78084 + t78462 - 0.49700494569958178265e-1_f64 * t70320 - t78464 - t78465 + t78469;
    t78470
}
