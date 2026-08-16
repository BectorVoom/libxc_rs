//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1025/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1025<F: Float>(t12630: F, t1820: F, t5125: F, t12650: F, t5018: F, t587: F, t10924: F, t2612: F, t10629: F, t2640: F, t12821: F, t16797: F, t639: F) -> (F, F, F, F, F) {
    let t42175 = t1820 * t5125 * t12630;
    let t42187 = t587 * t5018 * t12650;
    let t42189 = t2612 * t10924;
    let t42191 = t10629 * t2640;
    let t42204 = t639 * t16797 * t12821;
    (t42175, t42187, t42189, t42191, t42204)
}
