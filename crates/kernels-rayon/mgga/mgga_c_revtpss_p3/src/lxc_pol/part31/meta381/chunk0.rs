//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1418/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1418(t15827: f64, t4837: f64, t1659: f64, t3105: f64, t1062: f64, t4797: f64, t1660: f64, t3201: f64, t1058: f64, t4798: f64, t15127: f64, t15125: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15829 = 0.57165357490759649296e-3_f64 * t4837 * t15827;
    let t15830 = t1659 * t3105;
    let t15850 = t4797 * t1062;
    let t15862 = t1660 * t3201;
    let t15865 = 0.28582678745379824648e-3_f64 * t4798 * t1058;
    let t15874 = 0.37037037037037037037e-2_f64 * t15127;
    let t15875 = 0.11111111111111111111e-1_f64 * t15125;
    (t15829, t15830, t15850, t15862, t15865, t15874, t15875)
}
