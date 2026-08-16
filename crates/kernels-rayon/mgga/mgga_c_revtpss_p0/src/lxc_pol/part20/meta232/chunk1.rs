//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1029/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1029(t10861: f64, t827: f64, t828: f64, t821: f64, t235: f64) -> (f64, f64, f64, f64) {
    let t10863 = t827 * t828 * t10861;
    let t10866 = t821 * t821;
    let t10867 = 1.0_f64 / t10866;
    let t10868 = t10867 * t235;
    (t10863, t10866, t10867, t10868)
}
