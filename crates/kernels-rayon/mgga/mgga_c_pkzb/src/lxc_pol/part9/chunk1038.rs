//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1038/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1038(t1306: f64, t135: f64, t2457: f64, t273: f64, t3286: f64, t7920: f64, t7922: f64, t7924: f64, t7926: f64, t8006: f64, t8008: f64, t8011: f64, t8015: f64, t8019: f64, t8023: f64, t8025: f64, t8027: f64, t8030: f64, t8034: f64, t8241: f64, t8243: f64, t8291: f64, t8563: f64, t957: f64) -> f64 {
    let t8567 = t135 * t273 * t8563 * t957 - t1306 * t2457 * t3286 - t7920 - t7922 + t7924 + t7926 + t8006 - t8008 - t8011 - t8015 + t8019 + t8023 - t8025 - t8027 - t8030 + t8034 + t8241 + t8243 - t8291;
    t8567
}
