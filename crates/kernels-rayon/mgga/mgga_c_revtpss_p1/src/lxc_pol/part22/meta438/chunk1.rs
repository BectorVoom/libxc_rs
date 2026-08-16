//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2071/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2071(t14648: f64, t775: f64, t14832: f64, t2661: f64, t2652: f64, t4345: f64, t10716: f64, t4349: f64, t10746: f64, t10749: f64, t10756: f64, t10758: f64, t14817: f64, t14820: f64, t14823: f64, t14825: f64, t14829: f64, t2730: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14833 = t14648 * t775;
    let t14834 = t14832 * t14833;
    let t14836 = 0.28582678745379824648e-3_f64 * t2661 * t14834;
    let t14837 = t2652 * t4345;
    let t14839 = t10716 * t4349;
    let t14841 = 0.50820002809285328224e-5_f64 * t10746 - 0.36143185997963725432e-4_f64 * t10749 - 0.18071592998981862717e-4_f64 * t14817 + 0.25410001404642664112e-5_f64 * t14820 - t14823 + t2730 * t14825 / 8.0_f64 + t2730 * t14829 / 16.0_f64 - t14836 + 0.80031500487063509014e-2_f64 * t14837 + 0.54208002996571016773e-3_f64 * t14839 - t10756 - t10758;
    (t14833, t14834, t14836, t14837, t14839, t14841)
}
