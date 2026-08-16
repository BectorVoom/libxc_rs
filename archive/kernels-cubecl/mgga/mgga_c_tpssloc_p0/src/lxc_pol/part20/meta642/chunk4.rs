//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2354/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2354<F: Float>(t13783: F, t1597: F, t10237: F, t2986: F, t340: F, t4548: F, t698: F, t973: F, t10186: F, t10235: F, t13769: F, t13770: F, t13798: F, t13840: F, t13852: F, t13855: F, t42842: F, t43028: F, t43038: F, t48265: F, t48269: F) -> F {
    let t48279 = t13783 * t1597;
    let t48281 = t2986 * t48279 * t10237;
    let t48292 = t973 * t698 * t340 * t4548;
    let t48293 = F::cast_from(0.55555555555555555554e-3_f64) * t48292;
    let t48294 = F::cast_from(0.25925925925925925925e-2_f64) * t2986 * t13798 * t48265 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t10235 * t48269 + F::cast_from(0.59259259259259259257e-2_f64) * t10186 * t13840 + F::cast_from(0.55555555555555555554e-3_f64) * t43028 + F::cast_from(0.9259259259259259259e-4_f64) * t43038 + F::cast_from(0.29629629629629629629e-2_f64) * t10186 * t13770 - F::cast_from(0.37037037037037037036e-3_f64) * t48281 + F::cast_from(0.22222222222222222221e-2_f64) * t2986 * t13769 * t42842 + F::cast_from(0.44444444444444444443e-2_f64) * t10186 * t13852 + F::cast_from(0.22222222222222222221e-2_f64) * t10186 * t13855 + t48293;
    t48294
}
