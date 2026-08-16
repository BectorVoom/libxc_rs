//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1453/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1453<F: Float>(t1036: F, t4617: F, t10422: F, t4574: F, t3070: F, t1597: F, t4509: F) -> (F, F, F, F) {
    let t13758 = t4617 * t1036 / F::cast_from(2304.0_f64);
    let t13765 = t10422 * t4574;
    let t13767 = t3070 * t13765 / F::cast_from(3456.0_f64);
    let t13769 = t4509 * t1597;
    (t13758, t13765, t13767, t13769)
}
