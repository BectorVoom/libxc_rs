//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 353/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk353(t2850: f64, t2901: f64, t1035: f64, t702: f64, t1024: f64, t779: f64, t2513: f64, t2515: f64, t2520: f64, t2522: f64, t1020: f64, t471: f64, t64: f64) -> (f64, f64, f64, f64, f64) {
    let t2902 = t2850 + t2901;
    let t2909 = t1035 * t702;
    let t2912 = t779 * t1024;
    let t2919 = -21.0_f64 / 128.0_f64 * t2513 + 21.0_f64 / 4096.0_f64 * t2515 - 7.0_f64 / 4096.0_f64 * t2520 + 7.0_f64 / 128.0_f64 * t2522;
    let t2925 = t2919 * t471 - 4.0_f64 / 3.0_f64 * t1020 * t64 - 7.0_f64 / 128.0_f64 * t2513 + 7.0_f64 / 384.0_f64 * t2522;
    (t2902, t2909, t2912, t2919, t2925)
}
