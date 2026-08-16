//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 716/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk716<F: Float>(t101: F, t5880: F, t1590: F, t524: F, t142: F, t1378: F, t1971: F, t5701: F, t4579: F, t550: F, t553: F, t1339: F, t4585: F) -> (F, F, F, F, F, F) {
    let t5881 = t101 * t5880;
    let t5887 = t524 * t1590;
    let t5888 = t5887 * t142;
    let t5891 = t5701 * t1378 * t1971;
    let t5895 = F::cast_from(0.59261670986728442646e-2_f64) * t550 * t4579 * t553;
    let t5898 = F::cast_from(0.14862827083471493416e-2_f64) * t1339 * t4585 * t1971;
    (t5881, t5887, t5888, t5891, t5895, t5898)
}
