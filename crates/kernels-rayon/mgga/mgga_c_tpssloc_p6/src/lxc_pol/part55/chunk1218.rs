//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1218/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1218(t26502: f64, t3701: f64, t1983: f64, t2019: f64, t24990: f64, t31047: f64, t25994: f64, t8526: f64, t26114: f64, t8327: f64, t33211: f64, t6535: f64) -> (f64, f64, f64, f64, f64) {
    let t120016 = t3701 * t26502;
    let t120019 = 2.0_f64 * t1983 * t2019 * t120016;
    let t120044 = 3.0_f64 * t1983 * t31047 * t24990;
    let t120063 = 4.0_f64 * t8526 * t25994;
    let t120067 = 2.0_f64 * t26114 * t8327;
    let t120069 = 4.0_f64 * t33211 * t6535;
    (t120019, t120044, t120063, t120067, t120069)
}
