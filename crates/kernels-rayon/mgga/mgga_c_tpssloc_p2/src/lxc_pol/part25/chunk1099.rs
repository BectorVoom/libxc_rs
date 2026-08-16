//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1099/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1099(t12303: f64, t221: f64, t26284: f64, t1361: f64, t26288: f64, t12255: f64, t3788: f64, t6936: f64, t22865: f64, t6604: f64, t6937: f64, t22776: f64, t22779: f64) -> (f64, f64, f64, f64, f64) {
    let t80931 = t26284 * t221 * t12303;
    let t80934 = t26288 * t1361 * t12303;
    let t80937 = t6936 * t3788 * t12255;
    let t80939 = t22865 * t6604;
    let t80940 = t80939 * t6937;
    let t80943 = t22779 * t22776;
    (t80931, t80934, t80937, t80940, t80943)
}
