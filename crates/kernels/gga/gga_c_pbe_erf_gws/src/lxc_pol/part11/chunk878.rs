//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 878/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk878<F: Float>(t1672: F, t211: F, t3391: F, t1663: F, t3443: F, t3563: F, t616: F, t1251: F, t3550: F, t3544: F, t3547: F, t2790: F, t7956: F, t1764: F, t1778: F, t3493: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32215 = t211 * t1672 * t3391;
    let t32260 = t3443 * t1663;
    let t32279 = t616 * t1672 * t3563;
    let t32373 = t1251 * t3550;
    let t32375 = t1251 * t3544;
    let t32405 = t1251 * t3547;
    let t32523 = t2790 * t7956;
    let t32629 = t3443 * t1764;
    let t32670 = t3493 * t1778;
    (t32215, t32260, t32279, t32373, t32375, t32405, t32523, t32629, t32670)
}
