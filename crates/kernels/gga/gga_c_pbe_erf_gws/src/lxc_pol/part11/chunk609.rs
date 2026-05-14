//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 609/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk609<F: Float>(t5852: F, t131: F, t120: F, t133: F, t1365: F, t4579: F, t550: F, t553: F, t1339: F, t1971: F, t4585: F, t2704: F, t2718: F, t7: F, t226: F, t542: F, t671: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5853 = 1.0 / t5852;
    let t5854 = t131 * t5853;
    let t5863 = 0.89405814814814814813e0 * t133 * t1365 * t120;
    let t5895 = 0.59261670986728442646e-2 * t550 * t4579 * t553;
    let t5898 = 0.14862827083471493416e-2 * t1339 * t4585 * t1971;
    let t5902 = 0.12833333333333333333e1 * t2704 - 20.0 / 27.0 * t2718;
    let t5903 = t5902 * M_PI;
    let t5904 = t5903 * t7;
    let t5906 = 4.0 / 3.0 * t226 * t5904;
    let t5917 = t542 * t671;
    (t5853, t5854, t5863, t5895, t5898, t5902, t5903, t5904, t5906, t5917)
}
