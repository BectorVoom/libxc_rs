//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1209/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1209<F: Float>(t35924: F, t35926: F, t35930: F, t35934: F, t35936: F, t35938: F, t35949: F, t35951: F, t35955: F, t31676: F, t31680: F, t31682: F, t31684: F, t31687: F, t35928: F, t35942: F, t35947: F, t35953: F) -> F {
    let t37786 = F::new(13.0) / F::new(144.0) * t35924;
    let t37787 = F::cast_from(0.64025200389650807212e-1_f64) * t35926;
    let t37789 = F::cast_from(0.85748036236139473944e-3_f64) * t35930;
    let t37790 = F::cast_from(0.42874018118069736972e-3_f64) * t35934;
    let t37791 = F::new(0.3973125e0) * t35936;
    let t37792 = F::new(0.264875e0) * t35938;
    let t37800 = F::cast_from(0.17149607247227894789e-2_f64) * t35949;
    let t37801 = F::cast_from(0.34299214494455789578e-2_f64) * t35951;
    let t37803 = F::cast_from(0.21437009059034868486e-3_f64) * t35955;
    let t37804 = t37786 + t37787 - F::cast_from(0.68598428988911579156e-2_f64) * t35928 - t37789 - t37790 - t37791 - t37792 - F::cast_from(0.68598428988911579156e-2_f64) * t31676 + F::cast_from(0.17149607247227894789e-2_f64) * t31680 - F::cast_from(0.34299214494455789578e-2_f64) * t35942 + F::cast_from(0.11181543925192587402e-1_f64) * t31682 - F::cast_from(0.12579236915841660828e-2_f64) * t31684 - F::cast_from(0.28582678745379824648e-3_f64) * t31687 + F::cast_from(0.17149607247227894789e-2_f64) * t35947 - t37800 - t37801 - F::cast_from(0.17149607247227894789e-2_f64) * t35953 + t37803;
    t37804
}
