//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1834/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1834(t10832: f64, t25245: f64, t25266: f64, t2648: f64, t2681: f64, t7036: f64, t820: f64, t839: f64, t25260: f64, t843: f64, t2726: f64, t10841: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93043 = t25245 * t10832;
    let t93045 = t25266 * t2648;
    let t93048 = t820 * t7036 * t2681;
    let t93049 = t93048 * t839;
    let t93054 = t820 * t25260 * t843;
    let t93055 = t93054 * t2726;
    let t93058 = t25245 * t10841;
    (t93043, t93045, t93048, t93049, t93055, t93058)
}
