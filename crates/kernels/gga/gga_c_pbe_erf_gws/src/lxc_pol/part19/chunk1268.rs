//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1268/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1268<F: Float>(t4083: F, t8743: F, t54616: F, t15084: F, t840: F, t2242: F, t4230: F, t15027: F, t9270: F, t15089: F, t4414: F, t14924: F) -> (F, F, F, F, F, F, F) {
    let t55884 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t8743 * t4083;
    let t55889 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t54616;
    let t55901 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t840 * t15084;
    let t55904 = t2242 * t4230;
    let t55918 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t9270 * t15027;
    let t55936 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t4414 * t15089;
    let t55942 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t4414 * t14924;
    (t55884, t55889, t55901, t55904, t55918, t55936, t55942)
}
