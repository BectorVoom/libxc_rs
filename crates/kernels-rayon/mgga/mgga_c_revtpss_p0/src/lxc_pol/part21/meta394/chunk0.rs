//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1846/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1846(t12810: f64, t3629: f64, t3626: f64, t221: f64, t462: f64, t68: f64, t461: f64, t1209: f64, t3766: f64, t5330: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12846 = t12810 * t3629;
    let t12847 = t3626 * t12846;
    let t12851 = t221 * t68 * t462;
    let t12853 = 5.0_f64 / 1296.0_f64 * t461 * t12851;
    let t12854 = t1209 * t3766;
    let t12855 = t12854 * t5330;
    (t12846, t12847, t12851, t12853, t12854, t12855)
}
