//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2282;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta629<F: Float>(t47160: F, t41291: F, t12932: F, t2427: F, t13133: F, t2430: F, t145: F, t185: F, t46191: F, t45872: F, t707: F, t12886: F, t706: F, t708: F, t41295: F, t157: F, t41284: F, t12940: F, t12923: F, t12939: F, t2244: F, t12892: F, t12908: F, t2250: F, t4194: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47161, t47162, t47164, t47166, t47168, t47171, t47172) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2282::<F>(t47160, t41291, t12932, t2427, t13133, t2430, t145, t185, t46191, t45872, t707, t12886, t706);
        let (t47174, t47175, t47178, t47181, t47183, t47185) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2283::<F>(t47172, t708, t41295, t157, t41284, t12940, t12923, t12939, t2244, t12892, t12908, t2250, t4194);
    (t47161, t47162, t47164, t47166, t47168, t47171, t47174, t47175, t47178, t47181, t47183, t47185)
}
