//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 726/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk726(t70500: f64, t7553: f64, t7555: f64, t290: f64, t70499: f64, t2012: f64, t7349: f64, t2019: f64, t640: f64, t68788: f64, t7764: f64, t2010: f64, t7755: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70502 = t7553 * t7555 * t70500;
    let t70504 = t290 * t70499;
    let t70506 = t7349 * t2012 * t70504;
    let t70510 = t2019 * t7764 * t640 * t68788;
    let t70512 = t290 * t68788;
    let t70514 = t2010 * t7755 * t70512;
    (t70502, t70504, t70506, t70510, t70512, t70514)
}
