//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta790 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2749;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta790<F: Float>(t41279: F, t5499: F, t12945: F, t4205: F, t46208: F, t4194: F, t5398: F, t607: F, t750: F, t46217: F, t13130: F, t32: F, t5519: F, t2659: F, t16606: F, t2379: F, t39463: F, t39468: F, t40714: F, t40716: F, t4314: F) -> (F, F, F, F, F, F, F, F) {
        let (t57959, t57961, t57962, t57966, t57970, t57972, t57973) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2749::<F>(t41279, t5499, t12945, t4205, t46208, t4194, t5398, t607, t750, t46217, t13130, t32, t5519);
        let (t57975, t57976) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2750::<F>(t2659, t57973, t16606, t2379, t39463, t39468, t40714, t40716, t4314, t57959, t57961, t57962, t57966, t57970, t57972);
    (t57959, t57961, t57962, t57966, t57970, t57972, t57975, t57976)
}
