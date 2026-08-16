//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2944/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2944(t3857: f64, t5567: f64, t1317: f64, t13672: f64, t2608: f64, t512: f64, t5566: f64, t1856: f64, t9544: f64, t13597: f64, t2516: f64, t2626: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48235 = t3857 * t5567;
    let t48237 = t1317 * t13672;
    let t48240 = t512 * t5566 * t2608;
    let t48243 = t512 * t1856 * t9544;
    let t48255 = t13597 * t2516;
    let t48260 = t13597 * t2626;
    (t48235, t48237, t48240, t48243, t48255, t48260)
}
