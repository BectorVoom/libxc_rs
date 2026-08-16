//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2599/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2599(t13969: f64, t22270: f64, t3506: f64, t11678: f64, t1227: f64, t15591: f64, t18301: f64, t18594: f64, t18955: f64, t18959: f64, t19051: f64, t22280: f64, t3577: f64, t3578: f64, t45030: f64, t45162: f64, t4582: f64, t4733: f64, t4974: f64, t5005: f64, t5012: f64, t5024: f64, t52600: f64, t52601: f64, t52610: f64, t5975: f64, t6221: f64, t6225: f64, t70330: f64, t72445: f64) -> f64 {
    let t72470 = t3506 * t13969 * t22270;
    let t72484 = -t52600 + t15591 * t6221 / 1024.0_f64 - 3.0_f64 / 256.0_f64 * t45030 * t4582 * t72445 * t18301 - t19051 * t4974 / 768.0_f64 - 5.0_f64 / 432.0_f64 * t1227 * t4582 * t52601 * t70330 - 5.0_f64 / 1728.0_f64 * t5005 * t18955 - t52610 + t5024 * t18959 / 144.0_f64 + t72470 / 768.0_f64 - t3577 * t3578 * t5012 * t5975 / 768.0_f64 - t45162 * t22280 / 768.0_f64 - t11678 * t3578 * t6225 * t4733 / 768.0_f64 + t5024 * t18594 / 48.0_f64;
    t72484
}
