//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1148/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1148<F: Float>(t2294: F, t2582: F, t6579: F, t6118: F, t6175: F, t1543: F, t1616: F, t5095: F, t785: F, t5174: F, t6407: F, t2120: F, t524: F, t6238: F, t2127: F, t546: F, t8028: F) -> (F, F, F, F, F, F, F) {
    let t20963 = t2582 * t2294 * t6579;
    let t20973 = t6118 * t6175;
    let t20989 = t5095 * t785 * t1616 * t1543;
    let t20991 = t6407 * t5174;
    let t20994 = t524 * t6238 * t2120;
    let t20995 = t20994 * t2127;
    let t20997 = t546 * t8028;
    (t20963, t20973, t20989, t20991, t20994, t20995, t20997)
}
