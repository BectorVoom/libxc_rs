//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2603/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2603(t1174: f64, t135: f64, t22128: f64, t22132: f64, t11665: f64, t1216: f64, t1227: f64, t15438: f64, t15507: f64, t18590: f64, t18955: f64, t19062: f64, t19072: f64, t21758: f64, t22158: f64, t3577: f64, t45128: f64, t4582: f64, t4984: f64, t5005: f64, t5024: f64, t50992: f64, t52759: f64, t65914: f64, t65920: f64, t65966: f64, t70330: f64) -> f64 {
    let t72597 = t1174 * t135 * t22128;
    let t72600 = t1174 * t135 * t22132;
    let t72622 = -t65966 * t4984 / 1024.0_f64 - t72597 / 864.0_f64 - t72600 / 144.0_f64 - t1227 * t4582 * t50992 * t70330 / 192.0_f64 + 5.0_f64 / 324.0_f64 * t5024 * t18955 - t15438 * t19062 / 1024.0_f64 + t52759 + t15507 * t19072 / 96.0_f64 - 7.0_f64 / 648.0_f64 * t65914 - t5005 * t18590 / 384.0_f64 - t65920 / 1152.0_f64 + 5.0_f64 / 4608.0_f64 * t11665 * t22158 - 5.0_f64 / 5184.0_f64 * t3577 * t45128 * t21758 * t1216;
    t72622
}
