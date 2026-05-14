//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 606/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk606<F: Float>(t158: F, t2670: F, t1054: F, t1790: F, t633: F, t1717: F, t183: F) -> (F, F, F, F) {
    let t2671 = t2670 * t158;
    let t2678 = t1790 * t1054;
    let t2679 = t2678 * t633;
    let t2682 = t1717 * t183;
    (t2671, t2678, t2679, t2682)
}
