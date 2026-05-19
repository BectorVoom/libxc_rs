//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 788/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk788<F: Float>(t4579: F, t550: F, t553: F, t1339: F, t1971: F, t4585: F, t2704: F, t2718: F, t7: F, t226: F, t1989: F, t679: F) -> (F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t5895 = F::cast_from(0.59261670986728442646e-2_f64) * t550 * t4579 * t553;
    let t5898 = F::cast_from(0.14862827083471493416e-2_f64) * t1339 * t4585 * t1971;
    let t5902 = F::cast_from(0.12833333333333333333e1_f64) * t2704 - F::new(20.0) / F::new(27.0) * t2718;
    let t5903 = t5902 * pi;
    let t5904 = t5903 * t7;
    let t5906 = F::new(4.0) / F::new(3.0) * t226 * t5904;
    let t5910 = t1989 * t679;
    (t5895, t5898, t5906, t5910)
}
