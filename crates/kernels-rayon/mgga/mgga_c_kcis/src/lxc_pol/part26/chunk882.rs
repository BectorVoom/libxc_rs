//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 882/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk882(t531: f64, t7086: f64, t833: f64, t3984: f64, t1380: f64, t6944: f64, t1444: f64, t6284: f64) -> (f64, f64, f64) {
    let t21063 = t7086 * t531;
    let t21064 = t21063 * t833;
    let t21065 = t3984 * t21064;
    let t21068 = t6944 * t1380;
    let t21069 = t3984 * t21068;
    let t21072 = t1444 * t6284;
    let t21073 = t21072 * t833;
    (t21065, t21069, t21073)
}
