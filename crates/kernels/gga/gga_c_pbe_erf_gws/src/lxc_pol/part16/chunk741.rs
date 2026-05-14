//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 741/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk741<F: Float>(t133: F, t5787: F, t1590: F, t524: F, t142: F, t1378: F, t1971: F, t5701: F, t4579: F, t550: F, t553: F, t1339: F, t4585: F, t2704: F, t2718: F, t7: F) -> (F, F, F, F, F, F) {
    let t5874 = t133 * t5787;
    let t5887 = t524 * t1590;
    let t5888 = t5887 * t142;
    let t5891 = t5701 * t1378 * t1971;
    let t5895 = 0.59261670986728442646e-2 * t550 * t4579 * t553;
    let t5898 = 0.14862827083471493416e-2 * t1339 * t4585 * t1971;
    let t5902 = 0.12833333333333333333e1 * t2704 - 20.0 / 27.0 * t2718;
    let t5903 = t5902 * M_PI;
    let t5904 = t5903 * t7;
    (t5874, t5888, t5891, t5895, t5898, t5904)
}
