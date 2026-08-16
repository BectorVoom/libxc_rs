//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1780/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1780(t3917: f64, t47506: f64, t2453: f64, t3908: f64, t4067: f64, t10115: f64, t1421: f64, t10168: f64, t3920: f64, t10147: f64, t4071: f64, t47472: f64, t47474: f64, t47478: f64, t47483: f64, t47487: f64, t47490: f64, t47493: f64, t47495: f64, t47497: f64, t47500: f64, t47504: f64) -> f64 {
    let t47507 = t47506 * t3917;
    let t47510 = t2453 * t4067 * t3908;
    let t47512 = t10115 * t1421;
    let t47516 = t10168 * t3920;
    let t47518 = -0.13170898365871023197e0_f64 * t47472 + 0.12142592671231907757e0_f64 * t47474 - 0.12142592671231907757e0_f64 * t47478 + 0.13878983423218070567e-1_f64 * t47483 + 0.18505311230957427423e-1_f64 * t47487 + 0.39029762157531132076e-1_f64 * t47490 - 0.13170898365871023197e0_f64 * t47493 - 0.1040793657534163522e-1_f64 * t47495 + 0.68293547082294194357e-1_f64 * t47497 + 0.7805952431506226415e-2_f64 * t47500 + t47504 - 0.11708928647259339623e0_f64 * t47507 + 0.69394917116090352835e-2_f64 * t47510 - 0.44178176337912614788e-3_f64 * t47512 - 0.26341796731742046395e1_f64 * t4071 * t10147 - 0.78059524315062264152e-1_f64 * t47516;
    t47518
}
