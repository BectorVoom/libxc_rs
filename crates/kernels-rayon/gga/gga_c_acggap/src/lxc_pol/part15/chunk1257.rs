//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1257/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1257(t31658: f64, t31660: f64, t32891: f64, t35924: f64, t37780: f64, t37781: f64, t37782: f64, t37787: f64, t37789: f64, t37790: f64, t40308: f64, t40310: f64, t40313: f64, t40316: f64, t40318: f64, t40322: f64, t40324: f64, t40326: f64) -> f64 {
    let t42046 = t37780 - t37781 + t37782 - 0.83861579438944405515e-2_f64 * t31658 + 0.94344276868812456207e-3_f64 * t31660 + t32891 + 0.17149607247227894789e-2_f64 * t40308 - 0.80031500487063509015e-2_f64 * t40310 + 13.0_f64 / 72.0_f64 * t35924 + t40313 / 12.0_f64 + t40316 / 12.0_f64 + t37787 + 0.34299214494455789578e-2_f64 * t40318 - 0.10718504529517434243e-2_f64 * t40322 + 0.12862205435420921092e-1_f64 * t40324 - t37789 - t37790 + 0.17149607247227894789e-1_f64 * t40326;
    t42046
}
