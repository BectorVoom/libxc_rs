//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 953/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk953(t7386: f64, t7389: f64, t5525: f64, t5560: f64, t5563: f64, t5566: f64, t5783: f64, t5790: f64, t7357: f64, t7393: f64, t7397: f64, t7401: f64) -> f64 {
    let t7434 = 0.32862666666666666666e0_f64 * t7386;
    let t7435 = 0.32862666666666666666e0_f64 * t7389;
    let t7442 = -0.29896666666666666667e0_f64 * t5525 + 0.39862222222222222223e0_f64 * t7357 - t7434 - t7435 + 0.24647e0_f64 * t7393 + 0.49294e0_f64 * t7397 + 0.24647e0_f64 * t7401 - t5783 - t5790 + 0.54771111111111111111e0_f64 * t5560 - 0.16431333333333333333e0_f64 * t5563 - 0.16431333333333333333e0_f64 * t5566;
    t7442
}
