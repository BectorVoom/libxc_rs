//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2959/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2959(t19150: f64, t4719: f64, t19167: f64, t4724: f64, t981: f64, t19471: f64, t18899: f64, t23451: f64, t41224: f64, t23648: f64, t4733: f64, t23568: f64, t3022: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78460 = 0.70178683471615754484e1_f64 * t4719 * t19150;
    let t78463 = 0.35089341735807877242e1_f64 * t981 * t4724 * t19167;
    let t78465 = 0.10389515463408878255e3_f64 * t4719 * t19471;
    let t78469 = 0.12304822629859687989e5_f64 * t981 * t41224 * t23451 * t18899;
    let t78472 = 0.6233709278045326953e3_f64 * t981 * t23648 * t4733;
    let t78474 = 0.51947577317044391277e2_f64 * t3022 * t23568;
    (t78460, t78463, t78465, t78469, t78472, t78474)
}
