//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1357/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1357(t120955: f64, t1983: f64, t6879: f64, t33234: f64, t6535: f64, t23938: f64, t7461: f64, t26977: f64, t25980: f64, t7042: f64, t33553: f64, t650: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120958 = 3.0_f64 * t1983 * t120955 * t6879;
    let t120962 = 2.0_f64 * t33234 * t6535;
    let t120964 = 2.0_f64 * t23938 * t7461;
    let t120966 = 2.0_f64 * t26977 * t7461;
    let t120968 = 2.0_f64 * t7042 * t25980;
    let t120973 = t650 * t33553;
    (t120958, t120962, t120964, t120966, t120968, t120973)
}
