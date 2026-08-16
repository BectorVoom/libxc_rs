//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta222 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1033;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta222(t225: f64, t5848: f64, t68: f64, t369: f64, t1539: f64, t1616: f64, t3071: f64, t1020: f64, t1041: f64, t1618: f64, t1622: f64, t3039: f64, t3070: f64, t3084: f64, t3130: f64, t3160: f64, t378: f64, t4572: f64, t4604: f64, t4625: f64, t4631: f64, t4641: f64, t4644: f64, t5857: f64, t5861: f64, t5869: f64, t5875: f64, t5880: f64, t5885: f64, t5890: f64, t5894: f64, t5900: f64, t973: f64, t349: f64, t1634: f64, t3174: f64, t381: f64, t5872: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5903, t5904, t5905, t5908, t5909, t5914) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1033(t225, t5848, t68, t369, t1539, t1616, t3071, t1020, t1041, t1618, t1622, t3039, t3070, t3084, t3130, t3160, t378, t4572, t4604, t4625, t4631, t4641, t4644, t5857, t5861, t5869, t5875, t5880, t5885, t5890, t5894, t5900, t973);
        let (t5915, t5919, t5920, t5928) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1034(t349, t5914, t1634, t3174, t381, t5872);
    (t5903, t5904, t5905, t5908, t5909, t5914, t5915, t5919, t5920, t5928)
}
