//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2173/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2173(t89880: f64, t89920: f64, t89957: f64, t90001: f64, t23858: f64, t7685: f64, t22607: f64, t7688: f64, t1390: f64, t16018: f64, t1983: f64, t6878: f64) -> (f64, f64, f64, f64) {
    let t90003 = t89880 + t89920 + t89957 + t90001;
    let t90020 = 2.0_f64 * t7685 * t23858;
    let t90022 = 3.0_f64 * t22607 * t7688;
    let t90023 = t1390 * t16018;
    let t90026 = 3.0_f64 * t1983 * t6878 * t90023;
    (t90003, t90020, t90022, t90026)
}
