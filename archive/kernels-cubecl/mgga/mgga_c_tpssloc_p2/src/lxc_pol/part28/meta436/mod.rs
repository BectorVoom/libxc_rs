//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1616;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta436<F: Float>(t23133: F, t849: F, t2707: F, t6621: F, t1891: F, t9223: F, t213: F, t1895: F, t1887: F, t206: F, t22715: F, t242: F, t6612: F, t812: F) -> (F, F, F, F, F, F, F, F) {
        let (t23134, t23135, t23136, t23138, t23140, t23143, t23145) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1616::<F>(t23133, t849, t2707, t6621, t1891, t9223, t213, t1895, t1887, t206, t22715, t242, t6612);
        let t23146 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1617::<F>(t23145, t812);
    (t23134, t23135, t23136, t23138, t23140, t23143, t23145, t23146)
}
