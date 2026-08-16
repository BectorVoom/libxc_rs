//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1427;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta278<F: Float>(t4098: F, t751: F, t172: F, t4095: F, t763: F, t1472: F, t2517: F, t1409: F, t9427: F, t2433: F, t3966: F, t9438: F) -> (F, F, F, F, F, F, F) {
        let (t12850, t12858, t12860, t12861, t12862, t12865, t12874) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1427::<F>(t4098, t751, t172, t4095, t763, t1472, t2517, t1409, t9427, t2433, t3966, t9438);
    (t12850, t12858, t12860, t12861, t12862, t12865, t12874)
}
