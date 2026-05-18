//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 920/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk920<F: Float>(t1216: F, t1314: F, t470: F, t4734: F, t4737: F, t1215: F, t457: F, t4619: F, t1327: F, t1333: F, t1319: F, t1322: F, t18563: F) -> (F, F, F, F) {
    let t18933 = F::new(0.61523382126046769581e4) * t470 * t4734 * t1216 * t4737 * t1314;
    let t18939 = F::new(0.46785787179641632568e1) * t470 * t1215 * t4619 * t457;
    let t18941 = F::new(120.0) * t1333 * t1327;
    let t18950 = F::new(0.51947267698127589897e2) * t470 * t1319 * t18563 * t1322;
    (t18933, t18939, t18941, t18950)
}
