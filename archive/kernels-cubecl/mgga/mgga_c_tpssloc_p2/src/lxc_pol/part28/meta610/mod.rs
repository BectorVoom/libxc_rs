//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1921;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta610<F: Float>(t22724: F, t26436: F, t1307: F, t1352: F, t1834: F, t22633: F, t6976: F, t16037: F, t1992: F, t22897: F, t26423: F, t81159: F, t215: F, t22839: F, t562: F, t80854: F, t16226: F, t22685: F, t26395: F, t3734: F, t6637: F, t16125: F) -> (F, F, F, F, F, F, F, F) {
        let (t90900, t90907, t90910, t90912) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1921::<F>(t22724, t26436, t1307, t1352, t1834, t22633, t6976, t16037, t1992, t22897, t26423, t81159);
        let (t90915, t90917, t90921, t90929) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1922::<F>(t215, t22839, t562, t80854, t16226, t22685, t26395, t3734, t6637, t16125, t1992, t6976);
    (t90900, t90907, t90910, t90912, t90915, t90917, t90921, t90929)
}
