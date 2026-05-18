//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 854/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk854<F: Float>(t10073: F, t4089: F, t10008: F, t10015: F, t10020: F, t10027: F, t10032: F, t10035: F, t10041: F, t10044: F, t10049: F, t10062: F, t10066: F, t10070: F, t1437: F, t213: F, t3924: F, t4004: F, t4087: F, t4118: F, t546: F, t5745: F, t820: F, t9840: F, t9891: F, t9899: F) -> F {
    let t10074 = t10073 * t4089;
    let t10076 = -F::new(0.58544643236296698113e-1) * t10015 - F::new(0.29272321618148349057e-1) * t10020 + F::new(0.58544643236296698113e-1) * t10027 - F::new(0.65854491829355115987e0) * t820 * t1437 * t9891 + F::new(0.21951497276451705329e-1) * t10032 + t10035 + F::new(0.39512695097613069591e1) * t5745 * t4087 * t9840 - F::new(0.16463622957338778996e-1) * t10041 - F::new(0.19514881078765566038e-2) * t10044 - F::new(0.19756347548806534796e1) * t820 * t4118 * t3924 + F::new(0.39512695097613069591e1) * t820 * t10049 * t4004 - F::new(0.65854491829355115987e0) * t820 * t1437 * t9899 + F::new(0.65854491829355115987e0) * t213 * t546 * t10008 - F::new(0.32927245914677557992e-1) * t10062 + F::new(0.16463622957338778996e-1) * t10066 - F::new(0.21951497276451705329e-1) * t10070 + F::new(0.19514881078765566038e-2) * t10074;
    t10076
}
