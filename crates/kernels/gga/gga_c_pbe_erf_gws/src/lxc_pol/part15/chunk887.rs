//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 887/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk887<F: Float>(t1017: F, t1764: F, t1403: F, t1827: F, t587: F, t1000: F, t1406: F, t1821: F, t1820: F, t197: F, t2620: F, t7355: F) -> (F, F, F, F) {
    let t7685 = t1017 * t1764;
    let t7686 = t7685 * t1403;
    let t7687 = t1827 * t7686;
    let t7689 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t587 * t7687;
    let t7690 = t1000 * t1406;
    let t7691 = t1821 * t7690;
    let t7693 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1820 * t7691;
    let t7694 = t2620 * t197;
    let t7695 = t7694 * t7355;
    (t7689, t7693, t7694, t7695)
}
