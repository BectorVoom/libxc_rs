//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2093;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta583<F: Float>(t374: F, t485: F, t486: F, t9697: F, t11778: F, t121: F, t1229: F, t204: F, t1090: F, t1227: F, t248: F, t11880: F, t44690: F, t11913: F, t11604: F, t496: F, t68: F, t107: F, t9576: F, t2585: F, t667: F, t106: F, t9364: F, t35761: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t45250, t45268, t45293, t45296, t45326) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2093::<F>(t374, t485, t486, t9697, t11778, t121, t1229, t204, t1090, t1227, t248, t11880, t44690);
        let (t45329, t45350, t45421, t45422, t45435, t45460) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2094::<F>(t11913, t44690, t11604, t496, t68, t107, t9576, t2585, t667, t106, t9364, t35761);
    (t45250, t45268, t45293, t45296, t45326, t45329, t45350, t45421, t45422, t45435, t45460)
}
