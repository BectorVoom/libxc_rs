//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3206/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3206<F: Float>(t12832: F, t17617: F, t12851: F, t1778: F, t17429: F, t17789: F, t12910: F, t12916: F, t17624: F, t11231: F, t12777: F, t12784: F, t12816: F, t12920: F, t17222: F, t17412: F, t17605: F, t17623: F, t17633: F, t17635: F, t17729: F, t3620: F, t3626: F, t3644: F, t3708: F, t3718: F, t3720: F, t44917: F, t44925: F, t44928: F, t44931: F, t5046: F, t5352: F, t5391: F, t57005: F, t57275: F) -> F {
    let t59142 = t12832 * t17617;
    let t59144 = t1778 * t12851;
    let t59146 = t17429 * t17789;
    let t59149 = t12910 * t12916 * t17624;
    let t59151 = -F::cast_from(0.3811023832717309953e-2_f64) * t17412 * t3620 - F::cast_from(0.7622047665434619906e-2_f64) * t5391 * t12816 + F::cast_from(0.22866142996303859718e-2_f64) * t17605 * t12777 + F::cast_from(0.19055119163586549765e-3_f64) * t44917 + F::cast_from(0.64311027177104605458e-3_f64) * t3708 * t17222 + F::cast_from(0.12862205435420921092e-2_f64) * t12910 * t3720 * t17633 * t17623 - F::cast_from(0.64311027177104605458e-3_f64) * t3718 * t3720 * t57275 * t5352 + t44925 / F::cast_from(432.0_f64) - t44928 / F::cast_from(864.0_f64) + F::cast_from(0.45732285992607719436e-2_f64) * t17412 * t3644 - F::cast_from(0.85748036236139473944e-3_f64) * t12784 * t17635 - F::cast_from(0.25724410870841842183e-2_f64) * t57005 * t3626 * t5046 * t11231 + F::cast_from(0.25724410870841842183e-2_f64) * t17729 * t3626 * t5046 * t12920 - F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t44931 - F::cast_from(0.85748036236139473944e-3_f64) * t59142 - F::cast_from(5.0_f64) / F::cast_from(486.0_f64) * t59144 - F::cast_from(0.85748036236139473944e-3_f64) * t59146 + F::cast_from(0.85748036236139473944e-3_f64) * t59149;
    t59151
}
