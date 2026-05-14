//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 891/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk891<F: Float>(t1500: F, t5602: F, t142: F, t2031: F, t5842: F, t1597: F, t1917: F, t528: F, t5420: F, t4551: F, t713: F, t1457: F, t762: F, t16487: F, t16490: F, t16503: F, t16508: F, t16512: F, t16515: F) -> (F, F, F) {
    let t18140 = t1500 * t5602;
    let t18144 = t2031 * t142 * t5842;
    let t18146 = t1597 * t1917;
    let t18149 = 0.19947266666666666666e0 * t528 * t5420;
    let t18150 = t4551 * t713;
    let t18152 = t1457 * t713;
    let t18155 = 0.26596355555555555555e0 * t762 * t1917;
    let t18156 = 0.39894533333333333332e0 * t18146 + t18149 + 0.19947266666666666666e0 * t18150 - 0.26596355555555555555e0 * t18152 - t18155 - t16487 - t16490 - t16503 + t16508 + t16512 - t16515;
    (t18140, t18144, t18156)
}
