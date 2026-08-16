//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 968/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk968(t1220: f64, t20454: f64, t6814: f64, t969: f64, t1835: f64, t4758: f64, t6820: f64, t10877: f64, t10893: f64, t10936: f64, t1212: f64, t15304: f64, t15362: f64, t15450: f64, t1831: f64, t18965: f64, t19042: f64, t3545: f64, t3585: f64, t3592: f64, t5211: f64, t5234: f64, t5247: f64, t5254: f64, t6789: f64, t6805: f64, t6808: f64) -> f64 {
    let t20455 = t20454 * t1220;
    let t20465 = t6814 * t969;
    let t20468 = t1835 * t4758;
    let t20471 = t6820 * t969;
    let t20474 = 2.0_f64 * t15362 * t1831 + 2.0_f64 * t5211 * t5234 - 2.0_f64 * t10936 * t6789 + 1.0_f64 * t3545 * t6805 + 1.0_f64 * t1212 * t20455 + 0.32164683177870697974e2_f64 * t10893 * t6808 + t19042 - 0.19751789702565206229e-1_f64 * t18965 - 0.23392893589820816284e1_f64 * t15304 * t5247 + 0.346315117987517266e2_f64 * t15450 * t5254 + 0.35089340384731224426e1_f64 * t3592 * t20465 - 0.23392893589820816284e1_f64 * t3585 * t20468 - 0.1038945353962551798e3_f64 * t10877 * t20471;
    t20474
}
