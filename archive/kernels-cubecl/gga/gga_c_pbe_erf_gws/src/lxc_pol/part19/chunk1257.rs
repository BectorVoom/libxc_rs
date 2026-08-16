//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1257/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1257<F: Float>(t55140: F, t829: F, t830: F, t4083: F, t8746: F, t2416: F, t4227: F, t353: F, t859: F, t938: F, t53424: F, t27047: F, t4216: F, t9296: F) -> (F, F, F, F, F, F) {
    let t55142 = t829 * t830 * t55140;
    let t55145 = t8746 * t4083;
    let t55151 = t2416 * t4227;
    let t55154 = t859 * t353 * t55151 * t938;
    let t55161 = F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t53424;
    let t55182 = t27047 * t9296 * t4216 * t938;
    (t55142, t55145, t55151, t55154, t55161, t55182)
}
