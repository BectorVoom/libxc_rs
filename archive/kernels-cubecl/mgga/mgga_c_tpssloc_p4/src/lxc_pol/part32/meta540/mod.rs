//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1884;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1885;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta540<F: Float>(t1714: F, t52: F, t2132: F, t24746: F, t4997: F, t7339: F, t5001: F, t7338: F, t1730: F, t7344: F) -> (F, F, F, F, F, F) {
        let (t27607, t27608, t27609, t27611, t27614) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1884::<F>(t1714, t52, t2132, t24746, t4997, t7339, t5001, t7338);
        let t27617 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1885::<F>(t1730, t7344);
    (t27607, t27608, t27609, t27611, t27614, t27617)
}
