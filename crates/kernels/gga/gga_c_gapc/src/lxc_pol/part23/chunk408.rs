//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 408/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk408<F: Float>(t705: F, t78: F, t238: F, t704: F, t233: F, t711: F, t712: F, t1165: F, t1167: F, t1169: F, t1197: F, t1199: F, t1201: F) -> (F, F, F, F, F) {
    let t2084 = t78 * t705;
    let t2088 = t704 * t238;
    let t2089 = F::cast_from(1.0_f64) / t2088;
    let t2090 = t233 * t2089;
    let t2091 = t711 * t711;
    let t2092 = t2091 * t712;
    let t2101 = -F::cast_from(0.78438333333333333333e0_f64) * t1165 + F::cast_from(0.15687666666666666667e1_f64) * t1167 + F::cast_from(0.68863333333333333333e0_f64) * t1169 + F::cast_from(0.14025833333333333333e0_f64) * t1197 + F::cast_from(0.28051666666666666667e0_f64) * t1199 + F::cast_from(0.17365833333333333333e0_f64) * t1201;
    (t2084, t2090, t2091, t2092, t2101)
}
