//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3268/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3268<F: Float>(t10770: F, t14547: F, t14894: F, t18444: F, t18469: F, t2724: F, t4362: F, t4364: F, t50943: F, t50947: F, t50954: F, t50966: F, t62012: F, t62015: F, t62021: F, t62029: F, t62033: F) -> F {
    let t62039 = F::cast_from(0.90357964994909313586e-5_f64) * t62012 - F::cast_from(0.45178982497454656791e-5_f64) * t62015 - F::cast_from(0.36590402022685436322e-3_f64) * t50943 - F::cast_from(0.10164000561857065645e-2_f64) * t50947 + F::cast_from(0.57165357490759649296e-4_f64) * t62021 + F::cast_from(0.10164000561857065645e-4_f64) * t50954 - F::cast_from(0.12862205435420921092e-2_f64) * t14894 * t4364 * t18444 * t14547 + F::cast_from(0.16006300097412701803e-1_f64) * t50966 - F::cast_from(0.10841600599314203355e-2_f64) * t62029 - F::cast_from(0.2032800112371413129e-3_f64) * t62033 + F::cast_from(0.85748036236139473944e-2_f64) * t4362 * t10770 * t18469 * t2724;
    t62039
}
