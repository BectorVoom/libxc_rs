//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 246/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk246<F: Float>(t2652: F, t2399: F, t313: F, t89: F, t1882: F, t842: F, t877: F, t681: F, t865: F, t311: F, t869: F) -> (F, F, F, F, F, F, F) {
    let t2793 = 4.0 / 9.0 * t2652;
    let t2816 = 4.0 / 27.0 * t89 * t2399 * t313;
    let t2817 = t1882 * t842;
    let t2819 = t1882 * t877;
    let t2823 = 4.0 / 27.0 * t2652;
    let t2839 = t89 * t681 * t865;
    let t2842 = 1.0 / t869 / t311;
    (t2793, t2816, t2817, t2819, t2823, t2839, t2842)
}
