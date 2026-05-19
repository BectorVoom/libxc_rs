//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1208/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1208<F: Float>(t2333: F, t2526: F, t795: F, t3262: F, t3263: F, t10918: F, t11625: F, t3275: F, t10610: F, t10611: F, t11479: F, t37483: F, t37488: F, t37495: F, t37499: F, t37503: F, t37507: F, t37524: F, t37528: F, t40479: F, t40483: F, t40485: F, t40490: F) -> (F, F, F, F) {
    let t40491 = t2333 * t2526;
    let t40492 = t40491 * t795;
    let t40495 = F::new(3.0) / F::new(2.0) * t3262 * t3263 * t40492;
    let t40502 = t3275 * t10918 * t11625;
    let t40505 = F::new(3.0) / F::new(2.0) * t10610 * t11479 * t10611;
    let t40506 = t40479 - F::cast_from(0.3903207359137154578e-3_f64) * t37483 - t40483 + F::cast_from(0.14905073231436680509e-2_f64) * t40485 + t40490 + t40495 + F::cast_from(0.36021158228745895953e-3_f64) * t37488 + F::cast_from(0.72042316457491791906e-3_f64) * t37495 - F::cast_from(0.51240438831339423711e-4_f64) * t37499 + F::cast_from(0.72042316457491791906e-3_f64) * t37503 - F::cast_from(0.10248087766267884742e-3_f64) * t37507 - t40502 - t40505 + t37524 - t37528;
    (t40495, t40502, t40505, t40506)
}
