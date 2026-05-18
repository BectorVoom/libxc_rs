//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 893/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk893<F: Float>(t155: F, t1660: F, t1665: F, t587: F, t5009: F, t5283: F, t1804: F, t1866: F, t1885: F, t5175: F, t1652: F, t5304: F) -> (F, F, F, F) {
    let t16942 = t155 * t1660;
    let t16944 = t587 * t16942 * t1665;
    let t16945 = F::new(16.0) / F::new(81.0) * t16944;
    let t16947 = t587 * t5283 * t5009;
    let t16948 = F::new(64.0) / F::new(27.0) * t16947;
    let t16953 = F::new(24.0) / F::new(5.0) * t587 * t1885 * t5175 * t1804 * t1866;
    let t16954 = t5304 * t1652;
    (t16945, t16948, t16953, t16954)
}
