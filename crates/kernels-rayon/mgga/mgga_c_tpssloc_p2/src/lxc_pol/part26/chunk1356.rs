//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1356/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1356(t2121: f64, t3427: f64, t7381: f64, t24574: f64, t24795: f64, t24799: f64, t3590: f64, t477: f64, t7365: f64, t85660: f64, t1170: f64, t24829: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t85941 = t2121 * t3427 * t7381;
    let t85943 = t24574 * t24795;
    let t85945 = t24574 * t24799;
    let t85947 = t477 * t3590;
    let t85952 = t85660 * t7365;
    let t85955 = t2121 * t1170 * t24829;
    (t85941, t85943, t85945, t85947, t85952, t85955)
}
