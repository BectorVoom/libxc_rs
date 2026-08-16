//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1423/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1423<F: Float>(t115352: F, t6897: F, t7700: F, t1377: F, t7936: F, t1307: F, t22633: F, t22635: F, t1992: F, t31558: F, t5353: F, t33310: F, t6883: F) -> (F, F, F, F) {
    let t122121 = t6897 * t115352 * t7700;
    let t122124 = t1377 * t7936;
    let t122127 = t22633 * t22635 * t122124 * t1307;
    let t122131 = t1992 * t22635 * t31558 * t5353;
    let t122133 = t6883 * t33310;
    (t122121, t122127, t122131, t122133)
}
