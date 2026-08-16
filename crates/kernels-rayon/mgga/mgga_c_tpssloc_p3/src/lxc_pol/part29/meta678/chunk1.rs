//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2271/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2271(t12725: f64, t12734: f64, t1442: f64, t1458: f64, t2314: f64, t24924: f64, t27879: f64, t4026: f64, t652: f64, t7271: f64, t7408: f64, t7989: f64, t90022: f64, t90026: f64, t90029: f64, t90034: f64, t90036: f64, t90038: f64, t90040: f64, t90051: f64, t90059: f64, t90062: f64, t90064: f64, t90068: f64, t90418: f64) -> f64 {
    let t94236 = -2.0_f64 * t1458 * t24924 * t652 - 4.0_f64 * t12725 * t7271 - 4.0_f64 * t12734 * t7989 - t1442 * t24924 - 4.0_f64 * t2314 * t27879 - 2.0_f64 * t4026 * t7408 + t90022 + t90026 - t90029 + t90034 - t90036 - t90038 + t90040 - t90051 - t90059 + t90062 + t90064 + t90068 + t90418;
    t94236
}
