//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1911;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1912;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta605<F: Float>(t1985: F, t7700: F, t80707: F, t214: F, t5318: F, t6888: F, t6891: F, t16065: F, t1992: F, t22897: F, t26378: F, t6914: F, t16044: F, t6976: F, t1372: F, t1799: F, t1307: F, t26331: F, t26446: F, t26411: F, t12420: F, t5335: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t90737, t90739, t90741, t90747, t90749) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1911::<F>(t1985, t7700, t80707, t214, t5318, t6888, t6891, t16065, t1992, t22897, t26378, t6914);
        let (t90752, t90754, t90757, t90759, t90763) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1912::<F>(t16044, t1992, t6976, t1372, t1799, t1307, t26331, t26446, t26411, t6914, t12420, t5335);
    (t90737, t90739, t90741, t90747, t90749, t90752, t90754, t90757, t90759, t90763)
}
