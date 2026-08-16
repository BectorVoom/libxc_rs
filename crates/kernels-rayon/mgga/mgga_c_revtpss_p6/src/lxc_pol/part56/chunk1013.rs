//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1013/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1013(t5389: f64, t72: f64, t3720: f64, t1287: f64, t1794: f64, t33485: f64, t1807: f64, t31993: f64, t1250: f64, t494: f64, t1828: f64, t8931: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34944 = t5389 * t72;
    let t34945 = t34944 * t3720;
    let t34949 = t33485 * t1794 * t1287;
    let t34952 = t31993 * t1807;
    let t34956 = t494 * t1794 * t1250;
    let t34957 = t3720 * t34956;
    let t34960 = t8931 * t1828;
    (t34944, t34945, t34949, t34952, t34956, t34957, t34960)
}
