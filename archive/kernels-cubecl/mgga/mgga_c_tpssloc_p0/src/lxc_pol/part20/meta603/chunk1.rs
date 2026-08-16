//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2184/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2184<F: Float>(t1174: F, t3561: F, t698: F, t11738: F, t11739: F, t248: F, t3570: F, t10471: F, t44690: F, t11727: F, t44722: F, t44833: F, t44834: F, t478: F) -> (F, F, F, F, F) {
    let t44847 = t1174 * t698 * t3561;
    let t44851 = t11738 * t248 * t3570 * t11739;
    let t44857 = t44690 * t10471;
    let t44858 = t44857 * t11727;
    let t44863 = t44833 * t44722 * t478 * t44834;
    (t44847, t44851, t44857, t44858, t44863)
}
