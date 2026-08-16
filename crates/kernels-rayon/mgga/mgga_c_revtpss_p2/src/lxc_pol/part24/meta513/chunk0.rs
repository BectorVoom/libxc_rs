//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1530/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1530(t23862: f64, t3172: f64, t4837: f64, t1041: f64, t23822: f64, t11710: f64, t23920: f64, t3091: f64, t1058: f64, t23961: f64, t11859: f64, t11922: f64, t24008: f64) -> (f64, f64, f64, f64, f64) {
    let t79107 = t4837 * t3172 * t23862;
    let t79112 = t1041 * t3172 * t23822;
    let t79139 = t3091 * t11710 * t23920;
    let t79141 = t23961 * t1058;
    let t79155 = t11859 * t11922 * t24008;
    (t79107, t79112, t79139, t79141, t79155)
}
