//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta221(t3131: f64, t5872: f64, t1021: f64, t248: f64, t360: f64, t3151: f64, t5392: f64, t974: f64, t5398: f64, t998: f64, t3146: f64, t1044: f64, t5681: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5873, t5875, t5878, t5880, t5884, t5885, t5889, t5890, t5893, t5894, t5900) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1032(t3131, t5872, t1021, t248, t360, t3151, t5392, t974, t5398, t998, t3146, t1044, t5681);
    (t5873, t5875, t5878, t5880, t5884, t5885, t5889, t5890, t5893, t5894, t5900)
}
