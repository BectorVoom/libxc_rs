//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1569/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1569(t11289: f64, t1610: f64, t2869: f64, t4632: f64, t15125: f64, t15168: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15163: f64, t15166: f64, t15170: f64, t15173: f64) -> (f64, f64, f64, f64) {
    let t15425 = 1.0_f64 * t11289 * t1610;
    let t15427 = 2.0_f64 * t2869 * t4632;
    let t15435 = 0.39862222222222222222e0_f64 * t15125;
    let t15447 = 0.21908444444444444444e0_f64 * t15168;
    let t15450 = -0.19931111111111111111e0_f64 * t15137 - 0.33218518518518518518e0_f64 * t15142 + 0.11958666666666666667e1_f64 * t15147 + 0.59793333333333333334e0_f64 * t15151 + 0.11958666666666666667e1_f64 * t15156 - 0.17938e1_f64 * t15160 + 0.16431333333333333333e0_f64 * t15163 - 0.49293999999999999999e0_f64 * t15166 - t15447 + 0.36514074074074074074e-1_f64 * t15170 - 0.54771111111111111112e-1_f64 * t15173;
    (t15425, t15427, t15435, t15450)
}
