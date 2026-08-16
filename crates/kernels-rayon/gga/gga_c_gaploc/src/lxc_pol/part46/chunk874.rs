//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 874/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk874(t10557: f64, t30936: f64, t1: f64, t2392: f64, t3338: f64, t544: f64, t594: f64, t12987: f64, t7014: f64, t2365: f64, t31558: f64, t7025: f64) -> (f64, f64, f64, f64) {
    let t42250 = 0.17875244975925213335e2_f64 * t10557 * t30936;
    let t42254 = t544 * t594 * t3338 * t1 * t2392;
    let t42256 = t7014 * t12987;
    let t42257 = 0.15976219147466979032e-1_f64 * t42256;
    let t42259 = t7025 * t2365 * t31558;
    (t42250, t42254, t42257, t42259)
}
