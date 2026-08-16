//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 760/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk760(t160: f64, t162: f64, t1742: f64, t1747: f64, t1750: f64, t2631: f64, t5348: f64, t5357: f64, t5361: f64, t5364: f64, t594: f64, t597: f64) -> f64 {
    let t5367 = 60.0_f64 * t160 * t5357 + 3.0_f64 * t160 * t5364 - t162 * t5348 + 9.0_f64 * t1742 * t597 - 36.0_f64 * t1747 * t594 + 9.0_f64 * t1750 * t594 - 36.0_f64 * t2631 * t5361;
    t5367
}
