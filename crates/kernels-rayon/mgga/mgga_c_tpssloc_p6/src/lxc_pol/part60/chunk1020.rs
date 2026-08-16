//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1020/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1020(t2018: f64, t22574: f64, t24432: f64, t6347: f64, t2035: f64, t5493: f64, t1874: f64, t33234: f64, t7461: f64, t33617: f64, t4028: f64, t7458: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128401 = 3.0_f64 * t22574 * t24432 * t2018 * t6347;
    let t128402 = t2035 * t5493;
    let t128404 = 2.0_f64 * t128402 * t1874;
    let t128406 = 4.0_f64 * t33234 * t7461;
    let t128413 = 4.0_f64 * t4028 * t33617;
    let t128415 = 4.0_f64 * t7458 * t33617;
    (t128401, t128402, t128404, t128406, t128413, t128415)
}
