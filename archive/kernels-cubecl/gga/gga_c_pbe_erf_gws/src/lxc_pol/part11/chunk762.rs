//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 762/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk762<F: Float>(t12468: F, t1758: F, t11: F, t12339: F, t1663: F, t571: F, t2554: F, t3346: F, t12345: F, t572: F, t10823: F, t10825: F, t10827: F, t12462: F, t12466: F, t4940: F, t7374: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12469 = t1758 * t12468;
    let t12470 = t11 * t12469;
    let t12472 = t1663 * t12339;
    let t12473 = t571 * t12472;
    let t12474 = t11 * t12473;
    let t12476 = t2554 * t3346;
    let t12477 = t571 * t12476;
    let t12478 = t11 * t12477;
    let t12480 = t572 * t12345;
    let t12481 = t571 * t12480;
    let t12482 = t11 * t12481;
    let t12484 = t4940 + F::cast_from(0.25188888888888888889e-2_f64) * t7374 - F::cast_from(0.12594444444444444445e-2_f64) * t10823 + F::cast_from(0.37783333333333333335e-2_f64) * t10825 - F::cast_from(0.18891666666666666667e-2_f64) * t10827 + F::cast_from(0.20990740740740740742e-2_f64) * t12462 - F::cast_from(0.75566666666666666669e-2_f64) * t12466 + F::cast_from(0.37783333333333333335e-2_f64) * t12470 + F::cast_from(0.11335e-1_f64) * t12474 - F::cast_from(0.11335e-1_f64) * t12478 + F::cast_from(0.18891666666666666667e-2_f64) * t12482;
    (t12469, t12470, t12472, t12473, t12474, t12476, t12477, t12478, t12480, t12481, t12482, t12484)
}
