//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta702 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2288;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta702<F: Float>(t1227: F, t13969: F, t18593: F, t15640: F, t15737: F, t15503: F, t19025: F, t3535: F, t1202: F, t19032: F, t15498: F, t4993: F, t15486: F, t5024: F, t15590: F, t5018: F, t15507: F, t15548: F, t19057: F, t3506: F, t15438: F, t15569: F, t15608: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t66084, t66092, t66120, t66147, t66150, t66153) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2288::<F>(t1227, t13969, t18593, t15640, t15737, t15503, t19025, t3535, t1202, t19032, t15498, t4993);
        let (t66155, t66159, t66165, t66241, t66255, t66268) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2289::<F>(t15486, t5024, t15590, t5018, t15507, t15548, t13969, t19057, t3506, t15438, t15569, t15608);
    (t66084, t66092, t66120, t66147, t66150, t66153, t66155, t66159, t66165, t66241, t66255, t66268)
}
