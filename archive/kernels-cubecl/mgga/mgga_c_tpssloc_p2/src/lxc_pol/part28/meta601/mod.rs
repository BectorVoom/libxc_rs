//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1903;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta601<F: Float>(t22633: F, t22635: F, t26337: F, t3911: F, t26206: F, t6883: F, t1834: F, t794: F, t22892: F, t6891: F, t22704: F, t26355: F, t81326: F, t26197: F, t80670: F, t1307: F, t26331: F, t5187: F, t567: F, t26332: F, t3719: F, t213: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t90539, t90541, t90544, t90546, t90549) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1903::<F>(t22633, t22635, t26337, t3911, t26206, t6883, t1834, t794, t22892, t6891, t22704, t26355, t81326);
        let (t90551, t90556, t90560, t90566) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1904::<F>(t26197, t80670, t1307, t22635, t26331, t5187, t567, t26332, t3719, t1834, t213, t225);
    (t90539, t90541, t90544, t90546, t90549, t90551, t90556, t90560, t90566)
}
