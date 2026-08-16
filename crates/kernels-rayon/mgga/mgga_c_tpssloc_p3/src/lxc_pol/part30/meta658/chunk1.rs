//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2078/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2078(t90549: f64, t26197: f64, t80670: f64, t1834: f64, t213: f64, t225: f64, t22724: f64, t26474: f64, t22751: f64, t26194: f64, t1887: f64, t80830: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90550 = 0.16449340668482264365e-1_f64 * t90549;
    let t90551 = t80670 * t26197;
    let t90566 = t213 * t1834 * t225;
    let t90582 = t22724 * t26474;
    let t90584 = t22751 * t26194;
    let t90585 = 0.76763589786250567036e-1_f64 * t90584;
    let t90591 = t80830 * t1887;
    (t90550, t90551, t90566, t90582, t90585, t90591)
}
