//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2613/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2613(t11678: f64, t11697: f64, t22279: f64, t1227: f64, t15453: f64, t1735: f64, t18206: f64, t19077: f64, t22258: f64, t3490: f64, t3577: f64, t45020: f64, t45128: f64, t4582: f64, t4972: f64, t52836: f64, t53079: f64, t53097: f64, t53099: f64, t66268: f64, t66273: f64, t66276: f64, t66324: f64, t70316: f64, t70339: f64) -> f64 {
    let t72936 = t11678 * t11697 * t22279;
    let t72938 = t66268 / 216.0_f64 + t53079 / 3456.0_f64 + t53097 + t66273 / 54.0_f64 - t66276 / 288.0_f64 + t53099 / 3456.0_f64 - t3490 * t22258 / 768.0_f64 - t1227 * t4582 * t4972 * t70316 / 768.0_f64 + t45020 / 10368.0_f64 - 5.0_f64 / 1728.0_f64 * t1227 * t4582 * t15453 * t70339 - 5.0_f64 / 1728.0_f64 * t3577 * t45128 * t1735 * t18206 + t52836 * t19077 / 1024.0_f64 - 5.0_f64 / 648.0_f64 * t66324 - t72936 / 1152.0_f64;
    t72938
}
