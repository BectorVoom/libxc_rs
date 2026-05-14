//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 822/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk822<F: Float>(t16947: F, t1804: F, t1866: F, t1885: F, t5175: F, t587: F, t1652: F, t5304: F, t5309: F, t7136: F, t1898: F, t2704: F, t628: F, t1243: F, t1703: F, t1693: F) -> (F, F, F, F, F, F, F, F) {
    let t16948 = 64.0 / 27.0 * t16947;
    let t16953 = 24.0 / 5.0 * t587 * t1885 * t5175 * t1804 * t1866;
    let t16954 = t5304 * t1652;
    let t16955 = 64.0 / 45.0 * t16954;
    let t16957 = 16.0 / 5.0 * t7136 * t5309;
    let t16959 = 32.0 / 15.0 * t5304 * t1898;
    let t16960 = t2704 * t628;
    let t16962 = t1243 * t1703;
    let t16964 = t1243 * t1693;
    (t16948, t16953, t16955, t16957, t16959, t16960, t16962, t16964)
}
