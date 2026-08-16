//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1931/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1931(t28354: f64, t28430: f64, t858: f64, t218: f64, t28406: f64, t25224: f64, t7488: f64, t1880: f64, t1492: f64, t7510: f64, t17090: f64, t1912: f64, t23231: f64, t23252: f64, t23262: f64, t25206: f64, t25209: f64, t259: f64, t26712: f64, t26726: f64, t28307: f64, t28311: f64, t28317: f64, t4268: f64, t5637: f64, t5658: f64, t6627: f64, t7538: f64, t855: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28431 = t28354 + t28430;
    let t28432 = t858 * t28431;
    let t28437 = t218 * t28406;
    let t28439 = t25224 * t7488;
    let t28440 = t1880 * t28439;
    let t28442 = t1492 * t7510;
    let t28446 = -t23231 - t6627 * t5658 + 4.0_f64 * t855 * t28307 - 6.0_f64 * t855 * t28311 - 2.0_f64 * t4268 * t7538 + 2.0_f64 * t855 * t28317 + 0.82246703342411321824e-2_f64 * t25206 - t855 * t28432 + 0.76763589786250567036e-1_f64 * t25209 + t26712 + 2.0_f64 * t6627 * t5637 + t28437 * t259 + t23252 + t23262 - 0.16449340668482264365e-1_f64 * t28440 + 2.0_f64 * t28442 * t259 + t26726 - t17090 * t1912;
    (t28431, t28432, t28437, t28439, t28442, t28446)
}
