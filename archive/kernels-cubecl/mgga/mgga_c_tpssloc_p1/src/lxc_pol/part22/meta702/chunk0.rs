//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2288/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2288<F: Float>(t1227: F, t13969: F, t18593: F, t15640: F, t15737: F, t15503: F, t19025: F, t3535: F, t1202: F, t19032: F, t15498: F, t4993: F) -> (F, F, F, F, F, F) {
    let t66084 = t1227 * t13969 * t18593;
    let t66092 = t15737 * t15640;
    let t66120 = t15503 * t15640;
    let t66147 = t3535 * t19025;
    let t66150 = t1202 * t19032;
    let t66153 = t15498 * t4993;
    (t66084, t66092, t66120, t66147, t66150, t66153)
}
