//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2443/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2443(t10405: f64, t10410: f64, t10415: f64, t10863: f64, t10904: f64, t10937: f64, t13541: f64, t13982: f64, t13995: f64, t14130: f64, t14143: f64, t14147: f64, t14228: f64, t3048: f64, t3070: f64, t3071: f64, t4585: f64, t49929: f64, t49934: f64, t49940: f64, t49945: f64, t49957: f64, t49959: f64) -> f64 {
    let t49961 = t49929 * t10405 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t13995 * t10410 - t49934 * t10415 / 1536.0_f64 - t10904 * t13982 / 96.0_f64 + t49940 / 768.0_f64 + t10937 * t14130 / 144.0_f64 - t49945 / 1152.0_f64 + t10863 * t4585 / 72.0_f64 + t3048 * t14143 / 72.0_f64 + t3048 * t14147 / 144.0_f64 - t3070 * t3071 * t13541 * t14228 / 384.0_f64 + t49957 / 768.0_f64 - t49959 / 1536.0_f64;
    t49961
}
