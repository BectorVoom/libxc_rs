//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2029/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2029(t225: f64, t2627: f64, t236: f64, t25093: f64, t87229: f64, t1512: f64, t81807: f64, t81824: f64, t23041: f64, t4236: f64, t23040: f64, t4166: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87230 = t225 * t2627;
    let t87233 = t87229 * t87230 * t236 * t25093;
    let t87234 = 0.13457585364713463618e-3_f64 * t87233;
    let t87243 = t81807 * t1512;
    let t87247 = t81824 * t1512;
    let t87248 = 7.0_f64 / 1152.0_f64 * t87247;
    let t87255 = t23041 * t4236;
    let t87256 = 7.0_f64 / 1152.0_f64 * t87255;
    let t87261 = t4166 * t23040;
    (t87230, t87234, t87243, t87248, t87256, t87261)
}
