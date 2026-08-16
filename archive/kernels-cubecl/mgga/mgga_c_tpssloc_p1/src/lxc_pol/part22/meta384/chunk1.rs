//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1650/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1650<F: Float>(t18024: F, t3071: F, t1009: F, t5848: F, t1011: F, t1019: F) -> (F, F, F, F) {
    let t18025 = t3071 * t18024;
    let t18028 = t5848 * t1009;
    let t18029 = t18028 * t1011;
    let t18030 = t18029 * t1019;
    (t18025, t18028, t18029, t18030)
}
