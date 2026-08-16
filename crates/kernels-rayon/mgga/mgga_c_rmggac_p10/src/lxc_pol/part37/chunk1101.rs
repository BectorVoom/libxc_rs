//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1101/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1101(t305: f64, t69146: f64, t76171: f64, t76180: f64, t76182: f64, t76184: f64, t77860: f64, t77863: f64, t77864: f64, t77868: f64, t77869: f64, t77870: f64, t77873: f64, t80398: f64) -> f64 {
    let t80421 = -0.15531404553111930707e-1_f64 * t76171 - t77860 + t77863 + t77864 + 0.93188427318671584242e-2_f64 * t76180 - 0.15531404553111930707e-1_f64 * t76182 - 0.62125618212447722828e-2_f64 * t76184 + t77868 - t77869 - t77870 + 0.59871208509319042821e-1_f64 * t305 * t80398 - t77873 - t69146;
    t80421
}
