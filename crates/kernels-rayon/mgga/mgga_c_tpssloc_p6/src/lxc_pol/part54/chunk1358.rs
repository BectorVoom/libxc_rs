//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1358/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1358(t31759: f64, t7685: f64, t31300: f64, t91655: f64, t2018: f64, t22574: f64, t24432: f64, t5187: f64, t24995: f64, t37790: f64, t5308: f64, t2314: f64, t33617: f64) -> (f64, f64, f64, f64, f64) {
    let t120975 = 3.0_f64 * t7685 * t31759;
    let t120979 = 3.0_f64 * t91655 * t31300;
    let t120986 = 3.0_f64 * t22574 * t24432 * t2018 * t5187;
    let t120991 = 6.0_f64 * t24995 * t37790 * t5308;
    let t120993 = 2.0_f64 * t2314 * t33617;
    (t120975, t120979, t120986, t120991, t120993)
}
