//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1387/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1387<F: Float>(t34001: F, t34019: F, t34023: F, t34028: F, t34030: F, t34033: F, t34038: F, t34043: F, t34046: F, t34048: F, t34050: F, t34052: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36789 = F::cast_from(0.13493923611111111112e-4_f64) * t34001;
    let t36793 = F::cast_from(0.94685814672924837674e-4_f64) * t34019;
    let t36794 = F::cast_from(0.41030519691600762993e-3_f64) * t34023;
    let t36795 = F::cast_from(0.89759162297375602412e-9_f64) * t34028;
    let t36796 = F::cast_from(0.49239311888846044751e-7_f64) * t34030;
    let t36797 = F::cast_from(0.30890995649606120371e-4_f64) * t34033;
    let t36800 = F::cast_from(0.11594181388521408695e-4_f64) * t34038;
    let t36801 = F::cast_from(0.6154913986105755594e-8_f64) * t34043;
    let t36802 = F::cast_from(0.3077456993052877797e-8_f64) * t34046;
    let t36803 = F::cast_from(0.19888696349719110008e-6_f64) * t34048;
    let t36804 = F::cast_from(0.20633616410564056848e-4_f64) * t34050;
    let t36805 = F::cast_from(0.32017370162603252141e-6_f64) * t34052;
    (t36789, t36793, t36794, t36795, t36796, t36797, t36800, t36801, t36802, t36803, t36804, t36805)
}
