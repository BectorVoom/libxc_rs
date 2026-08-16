//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1444/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1444(t41338: f64, t41371: f64, t923: f64, t273: f64, t2881: f64, t2889: f64, t2897: f64, t41292: f64, t41299: f64, t41303: f64, t41307: f64, t41341: f64, t41344: f64, t41347: f64, t41350: f64, t41361: f64, t41363: f64, t41369: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41372 = t41338 + t41371;
    let t41373 = t923 * t41372;
    let t41382 = f64::powf(t273, -0.25e1_f64);
    let t41383 = t2881 * t2881;
    let t41384 = t41382 * t41383;
    let t41386 = t2889 * t2889;
    let t41387 = t2897 * t41386;
    let t41389 = 0.98115555555555555555e-1_f64 * t41292 - 0.8585111111111111111e-1_f64 * t41299 - 0.82785e-1_f64 * t41303 + t41307 + 0.16504875e0_f64 * t41373 - 0.89459259259259259259e0_f64 * t41341 - 0.301925e0_f64 * t41344 - 0.72462e1_f64 * t41347 + 0.40256666666666666666e1_f64 * t41350 + 0.12524296296296296297e1_f64 * t41361 + 0.16102666666666666667e1_f64 * t41363 - 0.16102666666666666667e1_f64 * t41369 + 0.6189328125e-1_f64 * t41384 + 0.247573125e0_f64 * t41387;
    (t41372, t41373, t41383, t41384, t41386, t41387, t41389)
}
