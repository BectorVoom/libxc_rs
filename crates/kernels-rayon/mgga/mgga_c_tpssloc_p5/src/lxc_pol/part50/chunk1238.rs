//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1238/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1238(t25994: f64, t8526: f64, t1874: f64, t90400: f64, t26114: f64, t8327: f64, t33211: f64, t6535: f64, t191: f64, t192: f64, t26138: f64, t2020: f64) -> (f64, f64, f64, f64, f64) {
    let t120063 = 4.0_f64 * t8526 * t25994;
    let t120064 = t90400 * t1874;
    let t120067 = 2.0_f64 * t26114 * t8327;
    let t120069 = 4.0_f64 * t33211 * t6535;
    let t120071 = t26138 * t191 * t192;
    let t120072 = t120071 * t2020;
    (t120063, t120064, t120067, t120069, t120072)
}
