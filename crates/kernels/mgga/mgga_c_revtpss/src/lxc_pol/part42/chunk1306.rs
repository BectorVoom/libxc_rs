//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1306/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1306<F: Float>(t22016: F, t22046: F, t5673: F, t5675: F, t1353: F, t6849: F, t800: F, t1872: F, t5591: F, t13804: F, t13959: F, t13987: F, t13988: F, t14001: F, t14007: F, t22156: F, t3944: F, t5671: F, t9748: F, t9804: F, t9847: F, t9910: F) -> (F,) {
    let t22159 = t5673 * t22046 * t22016;
    let t22163 = t5673 * t22046 * t5675;
    let t22169 = t800 * t6849 * t1353;
    let t22173 = t800 * t1872 * t5591;
    let t22176 = t9804 - 0.11337795902333997111e-1 * t13959 + 0.25410001404642664112e-5 * t9847 - 0.80031500487063509015e-2 * t22156 - 0.12862205435420921092e-2 * t13804 * t22159 + 0.12862205435420921092e-2 * t5671 * t22163 - 0.56688979511669985553e-2 * t9910 + t13987 - 0.80031500487063509015e-2 * t13988 - t14001 - t9748 * t22169 / 4.0 + t3944 * t22173 / 8.0 - t14007;
    (t22176,)
}
