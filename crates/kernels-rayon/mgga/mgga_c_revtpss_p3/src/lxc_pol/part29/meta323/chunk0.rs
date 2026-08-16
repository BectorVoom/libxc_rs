//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1231/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1231(t2719: f64, t820: f64, t843: f64, t2726: f64, t821: f64, t235: f64, t231: f64, t2723: f64, t2648: f64, t2741: f64, t2710: f64, t826: f64, t9732: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10858 = t820 * t2719 * t843;
    let t10859 = t10858 * t2726;
    let t10866 = t821 * t821;
    let t10867 = 1.0_f64 / t10866;
    let t10868 = t10867 * t235;
    let t10871 = t2723 * t231;
    let t10881 = t2741 * t2648;
    let t10885 = 0.81322168495418382223e-4_f64 * t2710 * t9732 * t826;
    (t10859, t10867, t10868, t10871, t10881, t10885)
}
