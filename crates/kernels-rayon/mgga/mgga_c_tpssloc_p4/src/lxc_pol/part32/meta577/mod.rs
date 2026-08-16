//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1954;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1955;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta577(t25: f64, t265: f64, t394: f64, t28755: f64, t1409: f64, t2116: f64, t28469: f64, t40: f64, t5398: f64, t7992: f64, t1760: f64, t8087: f64, t3598: f64, t2154: f64, t6267: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t7301: f64, t7300: f64, t2123: f64, t6140: f64, t1716: f64, t8010: f64, t27382: f64, t2130: f64, t46: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29507, t29514, t29532, t29535) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1954(t25, t265, t394, t28755, t1409, t2116, t28469, t40, t5398, t7992, t1760, t8087, t3598, t2154, t6267, dens_threshold, rho0, zeta_threshold);
        let (t29536, t29545, t29546, t29551, t29554, t29557, t29560) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1955(t29535, t3598, t6267, t7301, t7300, t2123, t6140, t1716, t8010, t27382, t2130, t46);
    (t29507, t29514, t29532, t29536, t29545, t29546, t29551, t29554, t29557, t29560)
}
