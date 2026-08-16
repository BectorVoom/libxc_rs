//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 876/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk876(t2155: f64, t7949: f64, t551: f64, t552: f64, t7591: f64, t5109: f64, t7356: f64, t2207: f64, t2208: f64, t2837: f64, t2612: f64, t495: f64) -> (f64, f64, f64, f64, f64) {
    let t7951 = 0.19514881078765566037e-1_f64 * t2155 * t7949;
    let t7953 = t551 * t552 * t7591;
    let t7956 = t5109 * t7356;
    let t7961 = t2207 * t2837 * t2208;
    let t7963 = t2612 * t495;
    (t7951, t7953, t7956, t7961, t7963)
}
