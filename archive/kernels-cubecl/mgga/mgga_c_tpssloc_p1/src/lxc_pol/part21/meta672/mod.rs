//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta672 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2476;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2477;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta672<F: Float>(t3540: F, t3567: F, t374: F, t485: F, t486: F, t9697: F, t11820: F, t3536: F, t1229: F, t204: F, t1090: F, t1227: F, t248: F, t3609: F, t44927: F, t3623: F, t11880: F, t44690: F, t11913: F, t11604: F, t496: F, t68: F, t107: F, t9576: F, t2585: F, t667: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t45224, t45250, t45266, t45293, t45296) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2476::<F>(t3540, t3567, t374, t485, t486, t9697, t11820, t3536, t1229, t204, t1090, t1227, t248);
        let (t45320, t45323, t45326, t45329, t45350, t45421, t45422) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2477::<F>(t3609, t44927, t3623, t11880, t44690, t11913, t11604, t496, t68, t107, t9576, t2585, t667);
    (t45224, t45250, t45266, t45293, t45296, t45320, t45323, t45326, t45329, t45350, t45421, t45422)
}
