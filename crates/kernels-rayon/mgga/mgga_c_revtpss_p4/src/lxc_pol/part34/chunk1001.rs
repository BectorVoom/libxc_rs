//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1001/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1001(t11534: f64, t15189: f64, t18919: f64, t18924: f64, t18934: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23501: f64, t23505: f64, t291: f64) -> f64 {
    let t23663 = -t11534 - 0.23744444444444444444e-1_f64 * t15189 + 0.11872222222222222222e-1_f64 * t18919 - 0.35616666666666666666e-1_f64 * t18924 + 0.17808333333333333333e-1_f64 * t18934 - 0.19787037037037037037e-1_f64 * t23479 + 0.71233333333333333332e-1_f64 * t23483 - 0.35616666666666666666e-1_f64 * t23501 - 0.10685e0_f64 * t23487 + 0.10685e0_f64 * t23505 - 0.17808333333333333333e-1_f64 * t23490;
    let t23665 = 0.621814e-1_f64 * t23663 * t291;
    t23665
}
