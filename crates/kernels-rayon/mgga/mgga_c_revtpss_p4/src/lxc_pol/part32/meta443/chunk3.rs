//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1613/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1613(t6871: f64, t9962: f64, t22016: f64, t22046: f64, t5673: f64, t5675: f64, t1353: f64, t6849: f64, t800: f64, t1872: f64, t5591: f64, t13804: f64, t13959: f64, t13987: f64, t13988: f64, t14001: f64, t14007: f64, t3944: f64, t5671: f64, t9748: f64, t9804: f64, t9847: f64, t9910: f64) -> (f64, f64, f64) {
    let t22156 = t9962 * t6871;
    let t22159 = t5673 * t22046 * t22016;
    let t22163 = t5673 * t22046 * t5675;
    let t22169 = t800 * t6849 * t1353;
    let t22173 = t800 * t1872 * t5591;
    let t22176 = t9804 - 0.11337795902333997111e-1_f64 * t13959 + 0.25410001404642664112e-5_f64 * t9847 - 0.80031500487063509015e-2_f64 * t22156 - 0.12862205435420921092e-2_f64 * t13804 * t22159 + 0.12862205435420921092e-2_f64 * t5671 * t22163 - 0.56688979511669985553e-2_f64 * t9910 + t13987 - 0.80031500487063509015e-2_f64 * t13988 - t14001 - t9748 * t22169 / 4.0_f64 + t3944 * t22173 / 8.0_f64 - t14007;
    (t22159, t22163, t22176)
}
