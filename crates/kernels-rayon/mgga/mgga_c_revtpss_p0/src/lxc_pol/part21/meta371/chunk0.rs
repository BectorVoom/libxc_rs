//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1760/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1760(t3434: f64, t421: f64, t12228: f64, t12227: f64, t1187: f64, t3495: f64, t3516: f64, t1196: f64, t1130: f64, t3376: f64, t1151: f64, t3379: f64, t3428: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12230 = 1.0_f64 / t3434 / t421;
    let t12231 = t12228 * t12230;
    let t12233 = 0.51726012919273400301e3_f64 * t12227 * t12231;
    let t12234 = t3495 * t1187;
    let t12235 = t12234 * t3516;
    let t12237 = 0.35089341735807877242e1_f64 * t1196 * t12235;
    let t12238 = t3376 * t1130;
    let t12240 = 3.0_f64 * t12238 * t1151;
    let t12242 = 3.0_f64 * t3379 * t3428;
    (t12230, t12231, t12233, t12235, t12237, t12238, t12240, t12242)
}
