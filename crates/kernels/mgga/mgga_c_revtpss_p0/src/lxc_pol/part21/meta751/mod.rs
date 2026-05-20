//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta751 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2628;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2629;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta751<F: Float>(t47133: F, t47135: F, t13665: F, t9572: F, t1320: F, t13680: F, t47145: F, t47147: F, t47149: F, t3863: F, t5569: F, t3860: F, t5571: F, t9419: F, t40076: F, t40079: F, t47131: F, t47138: F, t47140: F, t47142: F, t47152: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48322, t48323, t48325, t48327, t48328, t48329, t48330, t48332, t48333) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2628::<F>(t47133, t47135, t13665, t9572, t1320, t13680, t47145, t47147, t47149, t3863, t5569, t3860);
        let (t48334, t48336, t48337) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2629::<F>(t48333, t5571, t9419, t40076, t40079, t47131, t47138, t47140, t47142, t47152, t48322, t48323, t48325, t48327, t48328, t48329, t48330, t48332);
    (t48322, t48323, t48325, t48327, t48328, t48329, t48330, t48332, t48334, t48336, t48337)
}
