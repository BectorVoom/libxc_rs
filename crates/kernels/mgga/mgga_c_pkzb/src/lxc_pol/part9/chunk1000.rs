//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1000/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1000<F: Float>(t1613: F, t16588: F, t541: F, t555: F, t1692: F, t568: F, t5252: F, t6892: F, t146: F, t1540: F, t155: F, t52: F, t95: F, t1731: F, t5304: F, t1730: F) -> (F, F, F, F, F, F) {
    let t16950 = 0.35089341735807877242e1 * t555 * t1613 * t16588 * t541;
    let t17000 = t568 * t1692;
    let t17009 = t6892 * t5252;
    let t17026 = 455.0 / 243.0 * t146 / t52 / t1540 * t95 * t155;
    let t17033 = t1731 * t5304;
    let t17034 = t1730 * t17033;
    (t16950, t17000, t17009, t17026, t17033, t17034)
}
