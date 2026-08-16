//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2000;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2001;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta581(t22832: f64, t3777: f64, t1336: f64, t6943: f64, t836: f64, t3809: f64, t1995: f64, t1999: f64, t213: f64, t39041: f64, t557: f64, t6546: f64, t3766: f64, t1365: f64, t1878: f64, t22813: f64, t6924: f64, t80782: f64, t22794: f64, t22843: f64, t281: f64, t6597: f64, t1361: f64, t22690: f64, t3734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t80816, t80820, t80821, t80826, t80827) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2000(t22832, t3777, t1336, t6943, t836, t3809, t1995, t1999, t213, t39041, t557, t6546);
        let (t80828, t80830, t80836, t80837, t80840, t80843) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2001(t3766, t80827, t1365, t1878, t22813, t6924, t80782, t22794, t22843, t281, t6597, t1361, t22690, t3734);
    (t80816, t80820, t80821, t80826, t80827, t80828, t80830, t80836, t80837, t80840, t80843)
}
