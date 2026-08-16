//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 994/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk994(t23535: f64, t916: f64, t923: f64, t1600: f64, t6113: f64, t11354: f64, t11358: f64, t11334: f64, t11338: f64, t18919: f64, t18924: f64, t18934: f64, t19002: f64, t19004: f64, t19009: f64, t23521: f64, t23523: f64) -> (f64, f64, f64, f64, f64) {
    let t23536 = t916 * t23535;
    let t23538 = t923 * t23535;
    let t23540 = t6113 * t1600;
    let t23541 = t11354 * t23540;
    let t23543 = t11358 * t23540;
    let t23545 = 0.19931111111111111111e0_f64 * t18919 - 0.59793333333333333333e0_f64 * t18924 + 0.29896666666666666667e0_f64 * t18934 - t11334 - t11338 + 0.5477111111111111111e-1_f64 * t19002 - 0.32862666666666666666e0_f64 * t19004 + 0.16431333333333333333e0_f64 * t19009 - 0.28483875e1_f64 * t23521 + 0.46074375e0_f64 * t23523 + 0.1898925e1_f64 * t23536 + 0.3071625e0_f64 * t23538 + 0.142419375e1_f64 * t23541 - 0.76790625e-1_f64 * t23543;
    (t23536, t23538, t23541, t23543, t23545)
}
