//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2414;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta631<F: Float>(t2649: F, t41115: F, t2553: F, t2632: F, t10024: F, t809: F, t2614: F, t2693: F, t238: F, t244: F, t248: F, t40445: F, t212: F, t2586: F, t9523: F, t9525: F, t9577: F, t116: F, t2379: F, t207: F, t40419: F, t9538: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t41116, t41123, t41130, t41134, t41139) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2414::<F>(t2649, t41115, t2553, t2632, t10024, t809, t2614, t2693, t238, t244, t248, t40445);
        let (t41142, t41144, t41146, t41149, t41155) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2415::<F>(t212, t2553, t2586, t9523, t9525, t9577, t116, t244, t2379, t207, t40419, t9538);
    (t41116, t41123, t41130, t41134, t41139, t41142, t41144, t41146, t41149, t41155)
}
