//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1628;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta444(t23253: f64, t6562: f64, t225: f64, t258: f64, t2710: f64, t214: f64, t1880: f64, t1883: f64, t23012: f64, t23237: f64, t6572: f64, t213: f64, t252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23254, t23257, t23258, t23259, t23261, t23265, t23266, t23270) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1628(t23253, t6562, t225, t258, t2710, t214, t1880, t1883, t23012, t23237, t6572, t213, t252);
    (t23254, t23257, t23258, t23259, t23261, t23265, t23266, t23270)
}
