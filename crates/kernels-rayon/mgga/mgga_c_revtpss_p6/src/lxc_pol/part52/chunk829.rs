//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 829/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk829(t2475: f64, t72: f64, t245: f64, t136: f64, t853: f64, t220: f64, t821: f64, t866: f64, t2410: f64, t261: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10769 = t2475 * t72;
    let t10770 = t10769 * t245;
    let t10778 = t853 * t136;
    let t10779 = t10778 * t220;
    let t10866 = t821 * t821;
    let t10867 = 1.0_f64 / t10866;
    let t11006 = t866 * t866;
    let t11007 = 1.0_f64 / t11006;
    let t11064 = 1.0_f64 / t2410 / t261;
    (t10770, t10779, t10867, t11006, t11007, t11064)
}
