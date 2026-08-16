//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 716/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk716(t5135: f64, t66: f64, t168: f64, t167: f64, t180: f64, t173: f64, t1765: f64) -> (f64, f64, f64, f64) {
    let t5285 = 1.0_f64 / t66 / t5135;
    let t5286 = t168 * t5285;
    let t5289 = 0.37792653007779990369e-1_f64 * t167 * t5286 * t180;
    let t5295 = t1765 * t173;
    let t5296 = t167 * t5295;
    (t5286, t5289, t5295, t5296)
}
