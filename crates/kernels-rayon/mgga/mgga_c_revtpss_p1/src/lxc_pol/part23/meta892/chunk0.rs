//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2847/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2847(t76911: f64, t76929: f64, t150: f64, t190: f64, t162: f64, t187: f64, t61020: f64, t49866: f64, t39423: f64, t39425: f64, t39433: f64, t39438: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76930 = t76911 + t76929;
    let t76932 = t150 * t76930 * t190;
    let t76935 = 0.19751673498613801407e-1_f64 * t76930 * t162 * t187;
    let t76936 = 36.0_f64 * t61020;
    let t76937 = 0.30762056574649219972e4_f64 * t49866;
    let t76938 = 0.21687162600603479684e-1_f64 * t39423;
    let t76939 = 0.32530743900905219526e-1_f64 * t39425;
    let t76940 = 0.48159733137676571078e0_f64 * t39433;
    let t76941 = 0.16265371950452609763e-1_f64 * t39438;
    (t76932, t76935, t76936, t76937, t76938, t76939, t76940, t76941)
}
