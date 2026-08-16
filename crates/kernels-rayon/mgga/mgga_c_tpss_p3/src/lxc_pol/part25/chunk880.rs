//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 880/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk880(t2204: f64, t2214: f64, t719: f64, t123: f64, t173: f64, t186: f64, t2256: f64, t2267: f64, t2320: f64, t2327: f64, t2328: f64, t262: f64, t706: f64, t7829: f64, t7922: f64, t7929: f64, t7932: f64, t7936: f64, t7945: f64, t7946: f64, t7954: f64, t7960: f64, t7972: f64, t7975: f64, t7979: f64, t7988: f64, t7992: f64) -> f64 {
    let t7993 = t2204 * t2214;
    let t7994 = t7993 * t719;
    let t7997 = -0.48159733137676571078e0_f64 * t262 * t7922 * t2328 - t7929 + t7932 + t7936 - t7945 - 0.35089341735807877242e1_f64 * t2320 * t7946 + 0.16562821945185185185e-2_f64 * t123 * t7829 * t173 + t7954 + t7960 - t7972 - t7975 - t7979 + 0.56968947174242584612e-3_f64 * t123 * t7829 * t186 - 6.0_f64 * t2256 * t706 * t2267 - t7988 - t7992 + 0.51947577317044391277e2_f64 * t2327 * t7994;
    t7997
}
