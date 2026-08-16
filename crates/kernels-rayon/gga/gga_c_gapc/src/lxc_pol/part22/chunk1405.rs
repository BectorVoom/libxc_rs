//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1405/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1405(t34782: f64, t34785: f64, t34788: f64, t34791: f64, t34794: f64, t34804: f64, t34772: f64, t34776: f64, t34797: f64, t34802: f64, t37072: f64, t34808: f64) -> (f64, f64) {
    let t37073 = 0.20041830772435757309e-6_f64 * t34782;
    let t37074 = 0.69504740211613770836e-3_f64 * t34785;
    let t37075 = 0.50083268227528753081e-5_f64 * t34788;
    let t37076 = 0.43440462632258606772e-4_f64 * t34791;
    let t37077 = 0.11372686522837130914e-4_f64 * t34794;
    let t37080 = 0.9275345110817126956e-4_f64 * t34804;
    let t37081 = 0.19336854506021130163e-7_f64 * t34772 - 0.52389984474979915324e-9_f64 * t34776 - t37072 - t37073 - t37074 + t37075 + t37076 + t37077 + 0.29465683056794103106e-8_f64 * t34797 - 0.98332751566569010434e-8_f64 * t34802 + t37080;
    let t37082 = 0.77294542590142724634e-6_f64 * t34808;
    (t37081, t37082)
}
