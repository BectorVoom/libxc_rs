//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1383/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1383(t34019: f64, t34023: f64, t34028: f64, t34030: f64, t34033: f64, t34038: f64, t34043: f64, t34046: f64, t34048: f64, t34050: f64, t34052: f64, t34054: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36793 = 0.94685814672924837674e-4_f64 * t34019;
    let t36794 = 0.41030519691600762993e-3_f64 * t34023;
    let t36795 = 0.89759162297375602412e-9_f64 * t34028;
    let t36796 = 0.49239311888846044751e-7_f64 * t34030;
    let t36797 = 0.30890995649606120371e-4_f64 * t34033;
    let t36800 = 0.11594181388521408695e-4_f64 * t34038;
    let t36801 = 0.6154913986105755594e-8_f64 * t34043;
    let t36802 = 0.3077456993052877797e-8_f64 * t34046;
    let t36803 = 0.19888696349719110008e-6_f64 * t34048;
    let t36804 = 0.20633616410564056848e-4_f64 * t34050;
    let t36805 = 0.32017370162603252141e-6_f64 * t34052;
    let t36806 = 0.28605695478281987903e-5_f64 * t34054;
    (t36793, t36794, t36795, t36796, t36797, t36800, t36801, t36802, t36803, t36804, t36805, t36806)
}
