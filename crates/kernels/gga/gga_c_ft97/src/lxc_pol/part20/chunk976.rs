//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 976/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk976<F: Float>(t2832: F, t848: F, t10696: F, t863: F, t2749: F, t2770: F, t10478: F, t871: F, t2843: F, t10491: F, t10695: F, t311: F, t309: F, t1526: F, t2640: F, t42262: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44302 = t848 * t2832;
    let t44351 = t863 * t10696;
    let t44369 = t2770 * t2749;
    let t44518 = t10478 * t871;
    let t44523 = t2770 * t2843;
    let t44528 = t10491 * t871;
    let t44538 = t2770 * t2832;
    let t44600 = 1.0 / t10695 / t311;
    let t44601 = t309 * t44600;
    let t44663 = t1526 * t42262 * t2640;
    (t44302, t44351, t44369, t44518, t44523, t44528, t44538, t44600, t44601, t44663)
}
