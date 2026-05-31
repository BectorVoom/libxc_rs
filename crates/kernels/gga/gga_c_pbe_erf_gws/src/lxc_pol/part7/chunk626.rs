//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 626/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk626<F: Float>(t713: F, t762: F, t1597: F, t1917: F, t528: F, t1413: F, t1697: F, t617: F, t1809: F, t1620: F, t1698: F, t661: F) -> (F, F, F, F, F, F, F) {
    let t4872 = F::cast_from(0.66490888888888888888e-1_f64) * t762 * t713;
    let t4873 = t1597 * t713;
    let t4876 = F::cast_from(0.9973633333333333333e-1_f64) * t528 * t1917;
    let t4878 = t617 * t1697 * t1413;
    let t4879 = t1809 * t4878;
    let t4881 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1620 * t4879;
    let t4882 = t1698 * t661;
    (t4872, t4873, t4876, t4878, t4879, t4881, t4882)
}
