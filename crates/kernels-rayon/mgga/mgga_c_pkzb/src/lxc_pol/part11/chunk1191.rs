//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1191/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1191(t16901: f64, t24651: f64, t24653: f64, t20353: f64, t10534: f64, t46: f64, t552: f64, t16810: f64, t16813: f64, t16906: f64, t16909: f64, t16915: f64, t16923: f64, t20349: f64, t20352: f64, t20359: f64, t20360: f64, t20363: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29131 = 0.51947577317044391277e2_f64 * t16901;
    let t29132 = 12.0_f64 * t24651;
    let t29133 = 24.0_f64 * t24653;
    let t29134 = 0.30762056574649219972e4_f64 * t20353;
    let t29136 = t10534 * t46 * t552;
    let t29137 = 0.18311447306006545054e-3_f64 * t29136;
    let t29138 = -t29131 - t16906 + t16909 - t20349 + t16915 - t16923 - t29132 - t29133 + t20352 - t29134 - t20359 + t20360 - t20363 - t29137 + t16810 - t16813;
    (t29131, t29132, t29133, t29134, t29137, t29138)
}
