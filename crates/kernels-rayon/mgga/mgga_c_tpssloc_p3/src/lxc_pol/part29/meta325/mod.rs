//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta325(t1128: f64, t3324: f64, t1124: f64, t3356: f64, t3355: f64, t432: f64, t427: f64, t1094: f64, t3263: f64, t3395: f64, t3403: f64, t11135: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t11410, t11415, t11420, t11424, t11433, t11444) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1382(t1128, t3324, t1124, t3356, t3355, t432, t427, t1094, t3263, t3395, t3403, t11135);
    (t11410, t11415, t11420, t11424, t11433, t11444)
}
