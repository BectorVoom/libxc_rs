//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1227/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1227(t2664: f64, t9794: f64, t10760: f64, t2475: f64, t72: f64, t245: f64, t2482: f64, t814: f64, t823: f64, t136: f64, t853: f64, t220: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10761 = t9794 * t2664;
    let t10762 = t10760 * t10761;
    let t10769 = t2475 * t72;
    let t10770 = t10769 * t245;
    let t10777 = t2482 * t823 * t814;
    let t10778 = t853 * t136;
    let t10779 = t10778 * t220;
    (t10761, t10762, t10769, t10770, t10777, t10778, t10779)
}
