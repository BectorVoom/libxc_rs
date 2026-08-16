//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3295/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3295(t1892: f64, t6843: f64, t1399: f64, t1883: f64, t22009: f64, t46570: f64, t49199: f64, t49203: f64, t49210: f64, t5659: f64, t5755: f64, t74973: f64, t75113: f64, t75119: f64, t75123: f64, t75128: f64, t86455: f64) -> (f64, f64) {
    let t86506 = t1892 * t6843;
    let t86533 = t49199 - 0.91069445034239308177e-1_f64 * t49203 - 0.78059524315062264151e-2_f64 * t49210 - 0.19756347548806534796e1_f64 * t5755 * t22009 * t5659 - 0.39029762157531132074e-2_f64 * t75113 - 0.65854491829355115987e0_f64 * t5755 * t86455 * t1399 - 0.21951497276451705328e-1_f64 * t75119 - 0.34697458558045176418e-2_f64 * t75123 - 0.34697458558045176418e-2_f64 * t75128 - 0.19756347548806534796e1_f64 * t5755 * t74973 * t1883 + 0.17073386770573548589e-1_f64 * t46570;
    (t86506, t86533)
}
