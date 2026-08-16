//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 725/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk725(t57: f64, t2232: f64, t4573: f64, t4579: f64, t81: f64, t4733: f64, t150: f64, t190: f64, t3647: f64, t162: f64, t187: f64, t2208: f64, t2217: f64, t2224: f64, t2281: f64, t2285: f64, t2292: f64, t2302: f64, t2310: f64, t2333: f64, t2347: f64, t2351: f64, t4680: f64, t4682: f64, t4685: f64, t4686: f64, t4687: f64, t4727: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t155 = t57 <= zeta_threshold;
    let t4739 = piecewise3(t155, 0.0_f64, 4.0_f64 / 9.0_f64 * t2232 * t4573 - 4.0_f64 / 3.0_f64 * t81 * t4579);
    let t4740 = t4733 + t4739;
    let t4741 = t150 * t4740;
    let t4742 = t4741 * t190;
    let t4743 = 2.0_f64 * t3647;
    let t4744 = t4740 * t162;
    let t4746 = 0.19751673498613801407e-1_f64 * t4744 * t187;
    let t4747 = -t2208 - t2217 + t2224 + t2333 + t2302 + t2310 - t2292 + t4727 - t2281 + t2347 - t2285 - t4687 + t4742 + t4680 + t4682 + t2351 + t4743 + t4746 + t4685 - t4686;
    (t4740, t4741, t4742, t4743, t4744, t4746, t4747)
}
