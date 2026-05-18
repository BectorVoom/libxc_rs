//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 615/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk615<F: Float>(t174: F, t177: F, t838: F, t1660: F, t56: F, t1662: F, t259: F, t43: F, t155: F, t589: F) -> (F, F, F, F, F, F) {
    let t4939 = t174 * t838 * t177;
    let t4940 = F::new(0.58774074074074074074e-2) * t4939;
    let t4949 = t56 * t1660;
    let t4951 = F::new(1.0) / t1662 / t259;
    let t4957 = F::new(1.0) / t1662 / t43;
    let t4991 = t155 * t589;
    (t4939, t4940, t4949, t4951, t4957, t4991)
}
