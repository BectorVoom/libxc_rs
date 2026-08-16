//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1497/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1497(t56390: f64, t56392: f64, t56394: f64, t56398: f64, t54432: f64, t54434: f64, t193: f64, t20563: f64, t39570: f64, t39582: f64, t39585: f64, t39590: f64, t39593: f64, t39595: f64, t39597: f64, t5122: f64, t5126: f64, t6347: f64, t75256: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t79927 = 72.0_f64 * t56390;
    let t79928 = 192.0_f64 * t56392;
    let t79929 = 120.0_f64 * t56394;
    let t79930 = 6.0_f64 * t56398;
    let t79934 = 240.0_f64 * t54432;
    let t79935 = 0.20779030926817756511e3_f64 * t54434;
    let t79939 = 36.0_f64 * t193 * t6347 * t75256 + 72.0_f64 * t20563 * t5122 * t5126 + t39570 - t39582 - t39585 + t39590 - t39593 + t39595 - t39597 + t79927 + t79928 + t79929 + t79930 + t79934 - t79935;
    (t79927, t79928, t79929, t79930, t79934, t79935, t79939)
}
