//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1289/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1289<F: Float>(t113439: F, t113491: F, t100002: F, t100063: F, t100173: F, t100255: F, t106877: F, t106896: F, t106906: F, t106923: F, t106926: F, t106971: F, t1675: F, t23643: F, t23830: F, t23834: F, t23848: F, t23859: F, t23863: F, t23900: F, t23904: F, t23939: F, t25522: F, t27493: F, t27498: F, t27536: F, t6263: F, t6308: F, t6312: F, t93752: F, t93758: F, t93789: F, t93793: F) -> (F, F) {
    let t113492 = t113439 + t113491;
    let t113563 = -F::cast_from(0.14291339372689912324e-2_f64) * t25522 * t23848 + F::cast_from(0.17149607247227894789e-2_f64) * t27536 * t23863 - F::cast_from(0.17149607247227894789e-2_f64) * t100255 * t6263 + F::cast_from(0.85748036236139473944e-3_f64) * t106877 - F::cast_from(0.11433071498151929859e-2_f64) * t106896 + F::cast_from(0.11433071498151929859e-2_f64) * t106906 - F::cast_from(0.28582678745379824648e-3_f64) * t100002 - F::cast_from(0.11433071498151929859e-2_f64) * t106923 - F::cast_from(0.17149607247227894789e-2_f64) * t93752 * t23939 + F::cast_from(0.17149607247227894789e-2_f64) * t106926 + F::cast_from(0.25724410870841842183e-2_f64) * t100173 * t6308 + F::cast_from(0.25724410870841842183e-2_f64) * t93789 * t23830 - F::cast_from(0.25724410870841842183e-2_f64) * t93793 * t23834 - F::cast_from(0.12862205435420921092e-2_f64) * t100063 * t6312 + F::cast_from(0.42874018118069736972e-3_f64) * t93758 * t23643 - F::cast_from(0.85748036236139473944e-3_f64) * t25522 * t23859 + F::cast_from(0.17149607247227894789e-2_f64) * t27493 * t23900 - F::cast_from(0.85748036236139473944e-3_f64) * t27498 * t23904 + F::cast_from(0.85748036236139473944e-3_f64) * t106971 * t1675;
    (t113492, t113563)
}
