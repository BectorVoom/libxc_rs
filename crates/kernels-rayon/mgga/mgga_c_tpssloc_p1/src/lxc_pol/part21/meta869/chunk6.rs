//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3188/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3188(t15590: f64, t5018: f64, t15507: f64, t15548: f64, t1218: f64, t15438: f64, t15503: f64, t15531: f64, t15535: f64, t15555: f64, t15622: f64, t15627: f64, t18307: f64, t18346: f64, t3490: f64, t44858: f64, t44953: f64, t4980: f64, t52810: f64, t52836: f64, t52952: f64, t52973: f64, t52975: f64, t52987: f64, t53336: f64) -> f64 {
    let t66159 = t15590 * t5018;
    let t66165 = t15507 * t15548;
    let t66185 = -t66159 * t1218 / 144.0_f64 + t44953 / 10368.0_f64 - t52810 * t4980 / 72.0_f64 + t66165 / 216.0_f64 + t52952 / 3456.0_f64 - t44858 * t18307 / 256.0_f64 + 5.0_f64 / 1152.0_f64 * t3490 * t18346 - t52973 / 3456.0_f64 + t52975 / 324.0_f64 - t15438 * t15531 / 1536.0_f64 + t52836 * t15535 / 1536.0_f64 - t15503 * t15555 / 72.0_f64 - t15503 * t15622 / 144.0_f64 - t53336 * t15627 / 48.0_f64 + t52987 / 648.0_f64;
    t66185
}
