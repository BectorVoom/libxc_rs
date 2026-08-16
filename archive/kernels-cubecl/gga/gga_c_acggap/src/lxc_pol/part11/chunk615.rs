//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 615/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk615<F: Float>(t1532: F, t4711: F, t1181: F, t1545: F, t3431: F, t1524: F, t322: F, t1095: F, t398: F, t384: F, t1089: F, t1444: F, t429: F) -> (F, F, F, F, F, F) {
    let t4712 = t1532 * t4711;
    let t4713 = t1181 * t4712;
    let t4716 = t3431 * t1545;
    let t4718 = t1524 * t322;
    let t4720 = t398 * t1095 * t4718;
    let t4722 = F::cast_from(0.85748036236139473944e-3_f64) * t384 * t4720;
    let t4724 = t1089 * t429 * t1444;
    (t4713, t4716, t4718, t4720, t4722, t4724)
}
