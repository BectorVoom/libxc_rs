//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2410;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2411;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta629<F: Float>(t10108: F, t257: F, t68: F, t2627: F, t2710: F, t233: F, t9970: F, t2632: F, t2678: F, t9975: F, t2696: F, t9612: F, t10021: F, t812: F, t841: F, t849: F, t23076: F, t241: F, t67: F, t2707: F, t9601: F, t2703: F, t2559: F, t2570: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40890, t40895, t40931, t40933, t40951, t40961) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2410::<F>(t10108, t257, t68, t2627, t2710, t233, t9970, t2632, t2678, t9975, t2696, t9612);
        let (t40965, t40966, t40971, t40982, t40990, t41008) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2411::<F>(t10021, t812, t841, t849, t23076, t241, t67, t2707, t9601, t2703, t2559, t2570);
    (t40890, t40895, t40931, t40933, t40951, t40961, t40965, t40966, t40971, t40982, t40990, t41008)
}
