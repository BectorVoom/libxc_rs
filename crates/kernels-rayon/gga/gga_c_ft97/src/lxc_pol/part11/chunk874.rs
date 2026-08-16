//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 874/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk874(t37930: f64, t1619: f64, t7934: f64, t1597: f64, t62: f64, t66: f64, t22547: f64, t1620: f64, t6: f64, t7984: f64, t7988: f64, t5517: f64, t5544: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37931 = 1.0_f64 / t37930;
    let t37935 = t1619 * t7934;
    let t37939 = t1597 * t62;
    let t37940 = t37939 * t66;
    let t37941 = t22547 * t37940;
    let t37943 = t7984 * t6 * t1620;
    let t37947 = t7988 * t6 * t1620;
    let t37952 = t5517 * t5544;
    (t37931, t37935, t37941, t37943, t37947, t37952)
}
