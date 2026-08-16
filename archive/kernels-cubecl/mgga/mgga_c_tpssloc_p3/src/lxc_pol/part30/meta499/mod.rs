//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta499<F: Float>(t3127: F, t381: F, t23602: F, t1011: F, t1615: F, t4594: F, t1014: F, t1023: F, t1022: F, t7593: F, t1060: F, t1945: F, t4649: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25483, t25484, t25485, t25486, t25487, t25490, t25491, t25492, t25493, t25497, t25499) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1815::<F>(t3127, t381, t23602, t1011, t1615, t4594, t1014, t1023, t1022, t7593, t1060, t1945, t4649);
    (t25483, t25484, t25485, t25486, t25487, t25490, t25491, t25492, t25493, t25497, t25499)
}
