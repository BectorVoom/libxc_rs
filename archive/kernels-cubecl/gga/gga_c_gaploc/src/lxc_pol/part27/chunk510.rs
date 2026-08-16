//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 510/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk510<F: Float>(t2586: F, t738: F, t1841: F, t1897: F, t2504: F, t2508: F, t2509: F, t2533: F, t2538: F, t2542: F, t2545: F, t2550: F, t2556: F, t2560: F, t2565: F, t2573: F, t2577: F, t2583: F, t270: F, t650: F, t681: F, t938: F, t949: F) -> (F, F) {
    let t2587 = t738 * t2586;
    let t2590 = F::cast_from(0.10254034973522965712e-1_f64) * t650 * t938 + F::cast_from(0.76905262301422242837e-2_f64) * t681 * t938 - F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t2504 + F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t2509 + F::cast_from(0.76905262301422242837e-2_f64) * t270 * t2533 - F::cast_from(0.85450291446024714263e-3_f64) * t1841 * t2538 - F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t2542 - F::cast_from(0.42725145723012357132e-3_f64) * t2545 + F::cast_from(0.32043859292259267849e-3_f64) * t2550 - F::cast_from(0.32043859292259267849e-3_f64) * t2556 + F::cast_from(0.32043859292259267849e-3_f64) * t2560 - F::cast_from(0.32043859292259267849e-3_f64) * t2565 - F::cast_from(0.10254034973522965712e-1_f64) * t650 * t949 - F::cast_from(0.76905262301422242837e-2_f64) * t681 * t949 + F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t2573 + F::cast_from(0.85450291446024714263e-3_f64) * t1841 * t2577 + F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t2583 - F::cast_from(0.76905262301422242837e-2_f64) * t270 * t2587;
    (t2587, t2590)
}
