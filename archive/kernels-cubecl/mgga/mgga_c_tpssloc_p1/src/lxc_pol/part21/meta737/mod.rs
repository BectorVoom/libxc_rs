//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta737 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2598;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2599;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta737<F: Float>(t11698: F, t15569: F, t15498: F, t3523: F, t15495: F, t3572: F, t1227: F, t1653: F, t248: F, t45293: F, t15591: F, t15643: F, t3490: F, t1734: F, t3507: F, t11721: F, t11786: F, t5005: F, t15730: F, t3536: F, t15594: F, t1174: F, t14726: F, t44562: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52664, t52666, t52674, t52680, t52682, t52684) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2598::<F>(t11698, t15569, t15498, t3523, t15495, t3572, t1227, t1653, t248, t45293, t15591, t15643, t3490);
        let (t52696, t52704, t52725, t52731, t52733, t52751) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2599::<F>(t1734, t3507, t11721, t11786, t5005, t15730, t3536, t15594, t3523, t1174, t14726, t44562);
    (t52664, t52666, t52674, t52680, t52682, t52684, t52696, t52704, t52725, t52731, t52733, t52751)
}
