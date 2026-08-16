//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1372/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1372(t17353: f64, t17354: f64, t17301: f64, t17304: f64, t17308: f64, t17311: f64, t17333: f64, t17337: f64, t17339: f64, t17340: f64, t17342: f64, t17344: f64, t17347: f64, t17351: f64, t3674: f64, t484: f64) -> f64 {
    let t17355 = t17353 * t17354;
    let t17358 = -t17301 + 0.47637797908966374413e-4_f64 * t17304 + 0.42874018118069736972e-3_f64 * t17308 * t3674 - 0.11433071498151929859e-2_f64 * t17311 * t484 + 0.21437009059034868486e-3_f64 * t17333 * t484 - t17337 + t17339 + 0.2540682555144873302e-3_f64 * t17340 - 0.47637797908966374413e-4_f64 * t17342 - 0.12862205435420921092e-2_f64 * t17344 * t17347 + 0.28582678745379824648e-3_f64 * t17351 * t17355;
    t17358
}
