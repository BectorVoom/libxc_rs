//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta685 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta685(t19334: f64, t605: f64, t2235: f64, t5392: f64, t19534: f64, t88: f64, t1873: f64, t28007: f64, t6534: f64, t26114: f64, t7467: f64, t26117: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t96562, t96646, t96659, t96661, t96663, t96665) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2127(t19334, t605, t2235, t5392, t19534, t88, t1873, t28007, t6534, t26114, t7467, t26117);
    (t96562, t96646, t96659, t96661, t96663, t96665)
}
