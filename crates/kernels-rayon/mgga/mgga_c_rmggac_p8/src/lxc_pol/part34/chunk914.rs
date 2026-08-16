//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 914/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk914(t5148: f64, t75086: f64, t4669: f64, t74801: f64, t305: f64, t75141: f64, t76049: f64, t7788: f64, t76053: f64, t74802: f64, t7782: f64, t74806: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t76367 = t5148 * t75086;
    let t76368 = 0.15965655602485078085e0_f64 * t76367;
    let t76370 = 0.8980681276397856423e-1_f64 * t4669 * t74801;
    let t76372 = 0.2993560425465952141e-1_f64 * t305 * t75141;
    let t76373 = t7788 * t76049;
    let t76375 = t7788 * t76053;
    let t76377 = t7782 * t74802;
    let t76379 = t7782 * t74806;
    (t76368, t76370, t76372, t76373, t76375, t76377, t76379)
}
