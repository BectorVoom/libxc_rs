//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1705/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1705<F: Float>(t1113: F, t18221: F, t136: F, t18225: F, t6017: F, t699: F) -> (F, F, F, F, F) {
    let t18507 = t1113 * t18221;
    let t18508 = t136 * t18507;
    let t18509 = t1113 * t18225;
    let t18510 = t136 * t18509;
    let t18512 = t699 * t6017;
    (t18507, t18508, t18509, t18510, t18512)
}
