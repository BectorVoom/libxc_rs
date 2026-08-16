//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1194/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1194<F: Float>(t1413: F, t7712: F, t2310: F, t7630: F, t2001: F, t4728: F, t31849: F, t13287: F, t31195: F, t33953: F, t5270: F, t15386: F, t36323: F) -> (F, F, F, F, F, F) {
    let t36331 = t7712 * t1413;
    let t36332 = F::cast_from(0.85748036236139473944e-3_f64) * t36331;
    let t36333 = t7630 * t2310;
    let t36335 = t2001 * t4728;
    let t36340 = F::cast_from(0.15724046144802076034e-2_f64) * t31849;
    let t36344 = t31195 * t13287 * t33953 * t5270;
    let t36347 = t31195 * t15386 * t36323;
    (t36332, t36333, t36335, t36340, t36344, t36347)
}
