//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1954;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1955;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta577<F: Float>(t25: F, t265: F, t394: F, t28755: F, t1409: F, t2116: F, t28469: F, t40: F, t5398: F, t7992: F, t1760: F, t8087: F, t3598: F, t2154: F, t6267: F, dens_threshold: F, rho0: F, zeta_threshold: F, t7301: F, t7300: F, t2123: F, t6140: F, t1716: F, t8010: F, t27382: F, t2130: F, t46: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t29507, t29514, t29532, t29535) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1954::<F>(t25, t265, t394, t28755, t1409, t2116, t28469, t40, t5398, t7992, t1760, t8087, t3598, t2154, t6267, dens_threshold, rho0, zeta_threshold);
        let (t29536, t29545, t29546, t29551, t29554, t29557, t29560) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1955::<F>(t29535, t3598, t6267, t7301, t7300, t2123, t6140, t1716, t8010, t27382, t2130, t46);
    (t29507, t29514, t29532, t29536, t29545, t29546, t29551, t29554, t29557, t29560)
}
