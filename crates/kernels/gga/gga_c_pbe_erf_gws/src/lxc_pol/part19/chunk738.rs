//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 738/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk738<F: Float>(t4579: F, t550: F, t553: F, t1339: F, t1971: F, t4585: F, t2704: F, t2718: F, t7: F, t226: F, t1986: F, t666: F, t542: F, t671: F, t670: F, t1999: F, t245: F) -> (F, F, F, F, F, F) {
    let t5895 = 0.59261670986728442646e-2 * t550 * t4579 * t553;
    let t5898 = 0.14862827083471493416e-2 * t1339 * t4585 * t1971;
    let t5902 = 0.12833333333333333333e1 * t2704 - 20.0 / 27.0 * t2718;
    let t5903 = t5902 * M_PI;
    let t5904 = t5903 * t7;
    let t5906 = 4.0 / 3.0 * t226 * t5904;
    let t5912 = t666 * t1986;
    let t5917 = t542 * t671;
    let t5919 = 0.96187034332131941129e-1 * t670 * t5917;
    let t5920 = t245 * t1999;
    (t5895, t5898, t5906, t5912, t5919, t5920)
}
