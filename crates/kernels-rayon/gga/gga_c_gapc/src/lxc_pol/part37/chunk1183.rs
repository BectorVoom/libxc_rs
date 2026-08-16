//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1183/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1183(t11769: f64, t9703: f64, t3751: f64, t9422: f64, t11579: f64, t11849: f64, t2493: f64, t11853: f64, t19204: f64, t2578: f64, t3757: f64, t9638: f64) -> (f64, f64, f64, f64, f64) {
    let t33810 = t11769 * t9703;
    let t33812 = t3751 * t9422;
    let t33815 = t11849 * t11579 * t2493;
    let t33818 = t2578 * t19204 * t11853;
    let t33820 = t3757 * t9638;
    (t33810, t33812, t33815, t33818, t33820)
}
