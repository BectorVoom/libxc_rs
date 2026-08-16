//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1197/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1197(t19333: f64, t5393: f64, t1248: f64, t22346: f64, t2843: f64, t1091: f64, t15312: f64, t1901: f64, t22208: f64, t22405: f64, t22410: f64, t2857: f64, t2862: f64, t296: f64, t319: f64, t4246: f64, t44523: f64, t446: f64, t4965: f64, t5424: f64, t840: f64, t84331: f64, t84357: f64, t84390: f64, t84404: f64, t90313: f64) -> (f64, f64, f64) {
    let t90775 = t19333 * t5393;
    let t90785 = t2843 * t1248 * t22346;
    let t90799 = 2.0_f64 * t446 * t2862 * t319 * t90313 + 8.0_f64 / 9.0_f64 * t84331 - 8.0_f64 / 3.0_f64 * t1901 * t15312 * t22405 * t1091 - 2.0_f64 * t446 * t296 * t90775 + 8.0_f64 / 3.0_f64 * t1901 * t44523 * t22410 * t1091 - 8.0_f64 / 9.0_f64 * t84357 + 8.0_f64 / 3.0_f64 * t446 * t296 * t90785 + 4.0_f64 * t446 * t840 * t4246 * t22208 - 4.0_f64 / 9.0_f64 * t446 * t2857 * t5424 * t4965 + 8.0_f64 / 9.0_f64 * t84390 - 8.0_f64 / 3.0_f64 * t84404;
    (t90775, t90785, t90799)
}
