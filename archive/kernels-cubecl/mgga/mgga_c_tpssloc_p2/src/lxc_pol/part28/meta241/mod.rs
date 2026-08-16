//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1056;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1057;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta241<F: Float>(t6581: F, t805: F, t1891: F, t2230: F, t213: F, t1895: F, t202: F, t243: F, t598: F, t1894: F, t236: F, t776: F, t2229: F, t61: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t6582, t6584, t6586, t6589) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1056::<F>(t6581, t805, t1891, t2230, t213, t1895, t202, t243);
        let (t6590, t6591) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1057::<F>(t598, t6589, t213);
        let (t6593, t6594, t6597) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1058::<F>(t1894, t236, t776, t6591, t2229, t61);
    (t6582, t6584, t6586, t6589, t6590, t6591, t6593, t6594, t6597)
}
