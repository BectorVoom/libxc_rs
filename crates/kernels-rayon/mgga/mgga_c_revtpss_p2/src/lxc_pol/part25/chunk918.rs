//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 918/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk918(t10861: f64, t827: f64, t828: f64, t821: f64, t235: f64, t239: f64, t820: f64, t231: f64, t2723: f64, t10665: f64, t10666: f64, t2648: f64, t2741: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10863 = t827 * t828 * t10861;
    let t10866 = t821 * t821;
    let t10867 = 1.0_f64 / t10866;
    let t10868 = t10867 * t235;
    let t10870 = t820 * t10868 * t239;
    let t10871 = t2723 * t231;
    let t10872 = t10665 * t10871;
    let t10874 = t827 * t828 * t10872;
    let t10878 = t827 * t828 * t10666;
    let t10881 = t2741 * t2648;
    (t10863, t10867, t10870, t10871, t10872, t10874, t10878, t10881)
}
