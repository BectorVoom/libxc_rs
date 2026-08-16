//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2036/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2036(t86967: f64, t1484: f64, t2717: f64, t225: f64, t25051: f64, t23012: f64, t7489: f64, t23164: f64, t23204: f64, t25341: f64, t1887: f64, t81956: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86968 = 0.76763589786250567036e-1_f64 * t86967;
    let t86969 = t2717 * t1484;
    let t86988 = t25051 * t225;
    let t86991 = t23012 * t7489;
    let t87028 = t23164 * t23204 * t25341;
    let t87029 = 0.16449340668482264365e-1_f64 * t87028;
    let t87049 = t81956 * t1887;
    (t86968, t86969, t86988, t86991, t87029, t87049)
}
