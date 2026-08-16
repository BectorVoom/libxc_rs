//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1044/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1044(t10706: f64, t10719: f64, t10923: f64, t1364: f64, t14147: f64, t14151: f64, t14156: f64, t14157: f64, t14160: f64, t14162: f64, t14163: f64, t14165: f64, t14168: f64, t14426: f64, t198: f64, t207: f64, t2439: f64, t3552: f64, t750: f64, t7979: f64, t7988: f64, t7992: f64, t8222: f64, t8225: f64, t823: f64, t8231: f64, t8234: f64) -> f64 {
    let t14430 = t14426 * t198 * t207 * t823 + 6.0_f64 * t10923 * t1364 * t2439 + 6.0_f64 * t14151 * t3552 * t750 + t10706 - t10719 + t14147 + t14156 + t14157 + t14160 + t14162 + t14163 + t14165 + t14168 + t7979 + t7988 + t7992 + t8222 + t8225 - t8231 - t8234;
    t14430
}
