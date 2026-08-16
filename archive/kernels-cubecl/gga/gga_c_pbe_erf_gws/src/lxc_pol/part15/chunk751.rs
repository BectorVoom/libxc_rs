//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 751/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk751<F: Float>(t1770: F, t395: F, t1660: F, t56: F, t1662: F, t259: F, t43: F, t1783: F, t636: F, t1841: F, t735: F, t155: F, t589: F) -> (F, F, F, F, F, F, F) {
    let t4947 = t395 * t1770;
    let t4949 = t56 * t1660;
    let t4951 = F::cast_from(1.0_f64) / t1662 / t259;
    let t4957 = F::cast_from(1.0_f64) / t1662 / t43;
    let t4985 = t1783 * t636;
    let t4987 = t1841 * t735;
    let t4991 = t155 * t589;
    (t4947, t4949, t4951, t4957, t4985, t4987, t4991)
}
