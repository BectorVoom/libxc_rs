//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1562;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta280<F: Float>(t2563: F, t2610: F, t225: F, t2592: F, t2627: F, t852: F, t2710: F, t814: F, t856: F, t68: F, t2745: F, t870: F) -> (F, F, F, F, F, F, F, F) {
        let (t10038, t10049, t10054, t10076, t10108, t10109, t10110, t10126) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1562::<F>(t2563, t2610, t225, t2592, t2627, t852, t2710, t814, t856, t68, t2745, t870);
    (t10038, t10049, t10054, t10076, t10108, t10109, t10110, t10126)
}
