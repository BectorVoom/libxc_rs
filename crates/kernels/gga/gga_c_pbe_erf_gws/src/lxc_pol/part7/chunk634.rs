//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 634/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk634<F: Float>(t1660: F, t56: F, t1662: F, t259: F, t4352: F, t11: F, t43: F) -> (F, F, F, F, F, F) {
    let t4949 = t56 * t1660;
    let t4951 = F::new(1.0) / t1662 / t259;
    let t4952 = t4951 * t4352;
    let t4953 = t4949 * t4952;
    let t4954 = t11 * t4953;
    let t4957 = F::new(1.0) / t1662 / t43;
    (t4949, t4951, t4952, t4953, t4954, t4957)
}
