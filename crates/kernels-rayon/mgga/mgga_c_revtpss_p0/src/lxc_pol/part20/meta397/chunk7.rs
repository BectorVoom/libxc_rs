//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1472/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1472(t11409: f64, t11461: f64, t11525: f64, t2962: f64, t2971: f64, t3012: f64, t3014: f64, t41445: f64, t41464: f64, t41570: f64, t41573: f64, t41577: f64, t41580: f64, t41582: f64, t41585: f64, t41591: f64, t41657: f64, t41832: f64, t41841: f64, t41845: f64, t41847: f64, t41849: f64, t965: f64, t972: f64, t973: f64) -> f64 {
    let t41853 = -0.11579025239058625248e4_f64 * t11409 * t2971 * t2962 + t41570 + 0.2077903092681775651e3_f64 * t11461 * t11525 + 0.69263436422725855036e2_f64 * t3012 * t41832 * t972 + 0.5848223622634646207e0_f64 * t965 * t41445 * t973 - t41573 - t41577 - t41580 - t41582 - t41585 + t41591 - t41657 - t41841 - t41845 + t41847 - t41849 + 0.51947577317044391277e2_f64 * t3012 * t41464 * t3014;
    t41853
}
