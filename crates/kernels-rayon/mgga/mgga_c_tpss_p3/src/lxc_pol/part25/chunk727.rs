//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 727/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk727(t226: f64, t4758: f64, t773: f64, t774: f64, t4715: f64, t2389: f64, t4706: f64, t4701: f64, t801: f64, t2142: f64, t2147: f64, t2160: f64, t2173: f64, t2381: f64, t3615: f64, t3635: f64, t3681: f64, t4708: f64, t4712: f64, t4718: f64, t4724: f64, t761: f64, t771: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4759 = t4758 * t226;
    let t4761 = t773 * t774 * t4759;
    let t4764 = t4715 * t226;
    let t4766 = t773 * t774 * t4764;
    let t4771 = t2389 * t774 * t4706;
    let t4775 = t801 * t774 * t4701;
    let t4778 = t2142 + 7.0_f64 / 72.0_f64 * t3615 + t2147 * t4708 / 16.0_f64 - t761 * t4712 / 48.0_f64 + t2160 * t4718 / 1536.0_f64 + 7.0_f64 / 2304.0_f64 * t3635 + t2173 * t4724 / 384.0_f64 - t771 * t4761 / 3072.0_f64 - t771 * t4766 / 3072.0_f64 + t2381 + 7.0_f64 / 576.0_f64 * t3681 + 5.0_f64 / 768.0_f64 * t797 * t4771 - t797 * t4775 / 768.0_f64;
    (t4759, t4761, t4764, t4766, t4771, t4775, t4778)
}
