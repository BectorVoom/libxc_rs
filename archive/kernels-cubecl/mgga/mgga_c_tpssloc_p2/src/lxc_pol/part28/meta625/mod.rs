//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1950;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta625<F: Float>(t1985: F, t6907: F, t90739: F, t22685: F, t22686: F, t26193: F, t16018: F, t6888: F, t6889: F, t6890: F, t22674: F, t22892: F, t26189: F, t1324: F, t254: F, t22724: F, t26344: F, t22643: F, t7691: F, t81195: F, t1388: F, t25988: F, t1845: F, t3719: F, t22573: F, t7684: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t91469, t91478, t91482, t91486) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1950::<F>(t1985, t6907, t90739, t22685, t22686, t26193, t16018, t6888, t6889, t6890, t22674, t22892, t26189);
        let (t91505, t91531, t91548, t91565, t91603, t91655) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1951::<F>(t1324, t254, t22724, t26344, t22643, t7691, t81195, t1388, t25988, t1845, t3719, t22573, t7684);
    (t91469, t91478, t91482, t91486, t91505, t91531, t91548, t91565, t91603, t91655)
}
