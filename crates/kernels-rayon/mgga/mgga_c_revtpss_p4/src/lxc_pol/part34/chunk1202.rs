//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1202/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1202(t3057: f64, t93920: f64, t359: f64, t42066: f64, t3143: f64, t36870: f64, t1983: f64, t1981: f64, t42058: f64, t7143: f64, t11120: f64, t3140: f64) -> (f64, f64, f64, f64, f64) {
    let t93921 = t3057 * t93920;
    let t93968 = t42066 * t359;
    let t93982 = t36870 * t3143;
    let t93983 = t1983 * t93982;
    let t93994 = t1981 * t42058 * t7143;
    let t94014 = t3140 * t11120;
    (t93921, t93968, t93983, t93994, t94014)
}
