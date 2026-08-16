//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 772/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk772(t1297: f64, t1390: f64, t193: f64, t2426: f64, t2486: f64, t3819: f64, t3821: f64, t3825: f64, t3827: f64, t3832: f64, t5167: f64, t5169: f64, t5187: f64, t5263: f64, t5265: f64, t5267: f64, t5268: f64, t5269: f64, t533: f64, t5356: f64) -> f64 {
    let t5360 = t1390 * t193 * t533 * t5356 + 3.0_f64 * t1297 * t193 * t5187 - t2426 - t2486 + t3819 - t3821 + t3825 + t3827 - t3832 + t5167 + t5169 - t5263 + t5265 - t5267 - t5268 - t5269;
    t5360
}
