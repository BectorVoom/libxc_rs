//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1020/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1020<F: Float>(t1161: F, t3306: F, t2409: F, t3067: F, t1105: F, t1134: F, t858: F, t2407: F, t6672: F, t9016: F, t9127: F, t1114: F, t8987: F) -> (F, F, F, F, F, F) {
    let t11407 = t1161 * t3306;
    let t11409 = t2409 * t3067 * t11407;
    let t11412 = t1134 * t1105;
    let t11413 = t858 * t11412;
    let t11414 = t2407 * t11413;
    let t11416 = t6672 * t11414 / F::cast_from(24.0_f64);
    let t11418 = t9016 * t9127 / F::cast_from(24.0_f64);
    let t11419 = t1114 * t8987;
    (t11407, t11409, t11414, t11416, t11418, t11419)
}
