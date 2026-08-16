//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2384/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2384(t2661: f64, t2662: f64, t2749: f64, t40378: f64, t2430: f64, t853: f64, t837: f64, t836: f64, t124: f64, t2645: f64, t14686: f64, t14931: f64, t4366: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40553 = t2661 * t2662 * t40378 * t2749;
    let t40555 = t853 * t2430;
    let t40558 = t2661 * t2662 * t40555 * t837;
    let t40560 = t2430 * t836;
    let t40578 = t124 * t2645;
    let t40581 = t14931 * t14686 * t40578 * t4366;
    (t40553, t40555, t40558, t40560, t40578, t40581)
}
