//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3206/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3206(t12832: f64, t17617: f64, t12851: f64, t1778: f64, t17429: f64, t17789: f64, t12910: f64, t12916: f64, t17624: f64, t11231: f64, t12777: f64, t12784: f64, t12816: f64, t12920: f64, t17222: f64, t17412: f64, t17605: f64, t17623: f64, t17633: f64, t17635: f64, t17729: f64, t3620: f64, t3626: f64, t3644: f64, t3708: f64, t3718: f64, t3720: f64, t44917: f64, t44925: f64, t44928: f64, t44931: f64, t5046: f64, t5352: f64, t5391: f64, t57005: f64, t57275: f64) -> f64 {
    let t59142 = t12832 * t17617;
    let t59144 = t1778 * t12851;
    let t59146 = t17429 * t17789;
    let t59149 = t12910 * t12916 * t17624;
    let t59151 = -0.3811023832717309953e-2_f64 * t17412 * t3620 - 0.7622047665434619906e-2_f64 * t5391 * t12816 + 0.22866142996303859718e-2_f64 * t17605 * t12777 + 0.19055119163586549765e-3_f64 * t44917 + 0.64311027177104605458e-3_f64 * t3708 * t17222 + 0.12862205435420921092e-2_f64 * t12910 * t3720 * t17633 * t17623 - 0.64311027177104605458e-3_f64 * t3718 * t3720 * t57275 * t5352 + t44925 / 432.0_f64 - t44928 / 864.0_f64 + 0.45732285992607719436e-2_f64 * t17412 * t3644 - 0.85748036236139473944e-3_f64 * t12784 * t17635 - 0.25724410870841842183e-2_f64 * t57005 * t3626 * t5046 * t11231 + 0.25724410870841842183e-2_f64 * t17729 * t3626 * t5046 * t12920 - 5.0_f64 / 1296.0_f64 * t44931 - 0.85748036236139473944e-3_f64 * t59142 - 5.0_f64 / 486.0_f64 * t59144 - 0.85748036236139473944e-3_f64 * t59146 + 0.85748036236139473944e-3_f64 * t59149;
    t59151
}
