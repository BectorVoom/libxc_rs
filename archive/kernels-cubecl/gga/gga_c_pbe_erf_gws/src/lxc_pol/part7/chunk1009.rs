//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1009/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1009<F: Float>(t1472: F, t168: F, t1931: F, t153: F, t4867: F, t542: F, t5569: F, t703: F, t5: F, t922: F, t270: F, t4573: F, t745: F) -> (F, F, F, F, F, F) {
    let t18336 = t168 * t1472 * t1931;
    let t18339 = t153 * t542 * t4867;
    let t18342 = t168 * t703 * t5569;
    let t18344 = t5 * t922;
    let t18347 = F::cast_from(0.90790602394455990432e0_f64) * t168 * t18344 * t270;
    let t18349 = t153 * t4573 * t745;
    (t18336, t18339, t18342, t18344, t18347, t18349)
}
