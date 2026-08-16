//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1358/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1358(t120694: f64, t26161: f64, t26558: f64, t31670: f64, t7685: f64, t33363: f64, t6997: f64, t114360: f64, t120145: f64, t120148: f64, t120924: f64, t120926: f64, t120928: f64, t120930: f64, t120940: f64, t120941: f64, t2040: f64, t26872: f64, t27171: f64, t33085: f64, t6517: f64, t7050: f64) -> f64 {
    let t120944 = 2.0_f64 * t26161 * t26558 * t120694;
    let t120947 = t7685 * t31670;
    let t120948 = t33363 * t6997;
    let t120951 = -3.0_f64 * t114360 * t26872 - 2.0_f64 * t120145 * t2040 - 2.0_f64 * t120148 * t2040 - 2.0_f64 * t27171 * t6517 - 2.0_f64 * t33085 * t7050 - t120924 - t120926 - t120928 - t120930 + t120940 - t120941 + t120944 + t120947 + t120948;
    t120951
}
