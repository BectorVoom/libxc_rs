//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 994/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk994(t27453: f64, t27454: f64, t1751: f64, t477: f64, t1090: f64, t7362: f64, t1653: f64, t24858: f64, t2144: f64, t5011: f64, t1246: f64, t4733: f64, t7363: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27455 = t27453 * t27454;
    let t27460 = t477 * t1751;
    let t27461 = t27460 * t1090;
    let t27462 = t7362 * t27461;
    let t27465 = t24858 * t1653;
    let t27466 = t7362 * t27465;
    let t27470 = t2144 * t5011;
    let t27471 = t27470 * t1246;
    let t27473 = t7363 * t4733;
    (t27455, t27462, t27466, t27470, t27471, t27473)
}
