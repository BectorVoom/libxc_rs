//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta346(t13602: f64, t1553: f64, t2403: f64, t4392: f64, t699: f64, t13550: f64, t13563: f64, t1543: f64, t2791: f64, t2970: f64, t4343: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13603, t13642, t13644, t13645, t13650, t13675, t13679, t13709, t13712, t13727, t13750) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1389(t13602, t1553, t2403, t4392, t699, t13550, t13563, t1543, t2791, t2970, t4343, t973);
    (t13603, t13642, t13644, t13645, t13650, t13675, t13679, t13709, t13712, t13727, t13750)
}
