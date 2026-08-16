//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3176/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3176(t11738: f64, t15560: f64, t15564: f64, t15612: f64, t15617: f64, t18300: f64, t19077: f64, t3252: f64, t3494: f64, t3509: f64, t3516: f64, t3577: f64, t3578: f64, t44836: f64, t44965: f64, t45037: f64, t4582: f64, t4980: f64, t4984: f64, t5005: f64, t5024: f64, t52621: f64, t52628: f64, t52649: f64, t52653: f64, t52664: f64, t52903: f64, t53372: f64, t53399: f64, t6219: f64) -> f64 {
    let t65802 = -t52621 / 1728.0_f64 - t53399 * t4984 / 768.0_f64 + t53372 * t4980 / 384.0_f64 + t52649 / 3456.0_f64 - t5005 * t15612 / 1152.0_f64 + 5.0_f64 / 5184.0_f64 * t52653 + t52628 * t15560 / 216.0_f64 - t52903 * t15564 / 432.0_f64 + t52664 / 324.0_f64 - t3577 * t3578 * t6219 * t3252 / 4608.0_f64 + t44965 * t19077 / 1536.0_f64 + t11738 * t4582 * t18300 * t3494 / 3072.0_f64 + 7.0_f64 / 1536.0_f64 * t45037 * t4582 * t18300 * t3509 - t44836 * t4582 * t18300 * t3516 / 3072.0_f64 + t5024 * t15617 / 72.0_f64;
    t65802
}
