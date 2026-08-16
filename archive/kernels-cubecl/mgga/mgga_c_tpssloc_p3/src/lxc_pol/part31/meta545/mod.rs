//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1769;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1770;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta545<F: Float>(t22822: F, t281: F, t6589: F, t23124: F, t23076: F, t6597: F, t23047: F, t2617: F, t2690: F, t6612: F, t812: F, t831: F, t59: F, t9971: F, t23040: F, t23061: F, t6604: F, t1891: F, t1895: F, t213: F, t39041: F, t1887: F, t206: F, t80845: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t81788, t81789, t81792, t81803, t81807, t81808) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1769::<F>(t22822, t281, t6589, t23124, t23076, t6597, t23047, t2617, t2690, t6612, t812, t831);
        let (t81816, t81824, t81835, t81849, t81852) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1770::<F>(t59, t9971, t23040, t2617, t23061, t6604, t1891, t1895, t213, t39041, t1887, t206, t80845);
    (t81788, t81789, t81792, t81803, t81807, t81808, t81816, t81824, t81835, t81849, t81852)
}
