//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta171 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1096;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta171<F: Float>(t182: F, t4095: F, t145: F, t4094: F, t185: F, t1472: F, t751: F, t1409: F, t40: F, t52: F, t707: F, t75: F, t3966: F, t607: F, t767: F, t78: F, t771: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4097, t4098, t4099, t4100, t4101) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1096::<F>(t182, t4095, t145, t4094, t185, t1472, t751, t1409);
        let (t4102, t4103, t4104, t4111, t4119) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1097::<F>(t40, t52, t4101, t707, t1409, t75, t3966, t607, t767, t78, t771, zeta_threshold);
    (t4097, t4098, t4099, t4100, t4101, t4102, t4103, t4104, t4111, t4119)
}
