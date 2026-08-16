//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta219(t300: f64, t5797: f64, t5770: f64, t1589: f64, t4483: f64, t2904: f64, t5774: f64, t951: f64, t959: f64, t5790: f64, t942: f64, t2929: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5798, t5800, t5802, t5804, t5806, t5808, t5810, t5811) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1019(t300, t5797, t5770, t1589, t4483, t2904, t5774, t951, t959, t5790, t942, t2929);
    (t5798, t5800, t5802, t5804, t5806, t5808, t5810, t5811)
}
