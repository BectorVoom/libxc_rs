//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 630/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk630(t3020: f64, t381: f64, t1049: f64, t990: f64, t225: f64, t991: f64, t1008: f64, t191: f64) -> (f64, f64, f64, f64) {
    let t3021 = t3020 * t381;
    let t3023 = t990 * t1049;
    let t3026 = t991 * t225;
    let t3030 = 1.0_f64 / t1008 / t191;
    (t3021, t3023, t3026, t3030)
}
