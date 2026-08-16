//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1299/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1299(t31680: f64, t9239: f64, t22573: f64, t8606: f64, t111: f64, t8646: f64, t112: f64, t31781: f64, t580: f64, t1404: f64, t2022: f64, t7240: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t115907 = t9239 * t31680;
    let t115925 = t8606 * t22573;
    let t115984 = t8646 * t111;
    let t115996 = t31781 * t112;
    let t116014 = t31781 * t580;
    let t116021 = t8646 * t1404;
    let t116026 = t2022 * t7240;
    (t115907, t115925, t115984, t115996, t116014, t116021, t116026)
}
