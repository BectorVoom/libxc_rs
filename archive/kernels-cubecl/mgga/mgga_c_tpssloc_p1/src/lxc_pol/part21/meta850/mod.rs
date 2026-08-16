//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta850 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta850<F: Float>(t18746: F, t3279: F, t14758: F, t4764: F, t1102: F, t18730: F, t3287: F, t18751: F, t18754: F, t4748: F, t3270: F, t18761: F) -> (F, F, F, F, F, F, F, F) {
        let (t63848, t63853, t63856, t63858, t63860, t63862, t63865, t63867) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3078::<F>(t18746, t3279, t14758, t4764, t1102, t18730, t3287, t18751, t18754, t4748, t3270, t18761);
    (t63848, t63853, t63856, t63858, t63860, t63862, t63865, t63867)
}
