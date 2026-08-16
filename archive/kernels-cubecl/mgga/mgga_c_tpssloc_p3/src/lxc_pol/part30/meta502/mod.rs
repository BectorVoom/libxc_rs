//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta502<F: Float>(t7614: F, t968: F, t1920: F, t1948: F, t4657: F, t345: F, t4677: F, t6800: F, t6799: F, t4680: F, t1022: F, t1409: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25529, t25530, t25535, t25536, t25540, t25541, t25544, t25545, t25548) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1818::<F>(t7614, t968, t1920, t1948, t4657, t345, t4677, t6800, t6799, t4680, t1022, t1409);
    (t25529, t25530, t25535, t25536, t25540, t25541, t25544, t25545, t25548)
}
