//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2004/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2004(t3057: f64, t93920: f64, t25460: f64, t25698: f64, t1071: f64, t7150: f64, t8521: f64, t359: f64, t42066: f64, t3143: f64, t36870: f64, t1983: f64) -> (f64, f64, f64, f64, f64) {
    let t93921 = t3057 * t93920;
    let t93928 = t25698 * t25460;
    let t93962 = t7150 * t1071;
    let t93963 = t93962 * t8521;
    let t93968 = t42066 * t359;
    let t93982 = t36870 * t3143;
    let t93983 = t1983 * t93982;
    (t93921, t93928, t93963, t93968, t93983)
}
