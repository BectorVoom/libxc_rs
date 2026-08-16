//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 981/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk981(t1873: f64, t28951: f64, t3941: f64, t7467: f64, t7801: f64, t2098: f64, t5456: f64, t28017: f64, t7230: f64, t55388: f64, t8657: f64, t33211: f64, t7802: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127698 = 27.0_f64 * t3941 * t28951 * t1873;
    let t127701 = 54.0_f64 * t3941 * t7801 * t7467;
    let t127704 = t2098 * t5456;
    let t127706 = 27.0_f64 * t127704 * t1873;
    let t127708 = 0.135e2_f64 * t7230 * t28017;
    let t127714 = 27.0_f64 * t55388 * t8657;
    let t127720 = 4.0_f64 * t33211 * t7802;
    (t127698, t127701, t127706, t127708, t127714, t127720)
}
