//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3137/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3137(t12248: f64, t1732: f64, t12415: f64, t12222: f64, t5192: f64, t1196: f64, t45289: f64, t5205: f64, t12235: f64, t16673: f64, t3531: f64, t12361: f64, t16655: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57818 = t12248 * t1732;
    let t57820 = 0.2894756309764656312e3_f64 * t57818 * t12415;
    let t57822 = 0.51947577317044391277e2_f64 * t5192 * t12222;
    let t57825 = 0.17315859105681463759e2_f64 * t1196 * t5205 * t45289;
    let t57827 = 0.35089341735807877242e1_f64 * t5192 * t12235;
    let t57829 = 0.10389515463408878255e3_f64 * t3531 * t16673;
    let t57831 = 6.0_f64 * t12361 * t16655;
    (t57820, t57822, t57825, t57827, t57829, t57831)
}
