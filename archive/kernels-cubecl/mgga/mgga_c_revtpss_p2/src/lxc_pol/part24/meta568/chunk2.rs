//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1742/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1742<F: Float>(t5819: F, t6573: F, t6587: F, t6628: F, t1794: F, t24633: F, t6622: F, t1042: F, t1250: F, t12809: F, t12910: F, t1797: F, t1808: F, t24741: F, t3611: F, t3718: F, t3720: F, t5302: F, t5384: F, t57147: F, t82469: F, t82491: F, t82534: F, t82536: F, t83296: F, t83728: F) -> (F, F, F, F, F) {
    let t90037 = t5819 * t6573;
    let t90042 = t6587 * t6628;
    let t90054 = t24633 * t1794;
    let t90059 = t6573 * t6622;
    let t90066 = F::cast_from(0.34299214494455789578e-2_f64) * t82469 + F::cast_from(0.28582678745379824648e-2_f64) * t5384 * t1042 * t5302 * t90037 + F::cast_from(0.12862205435420921092e-2_f64) * t12809 * t3720 * t90042 * t3611 + F::cast_from(0.34299214494455789578e-2_f64) * t82491 - F::cast_from(0.27439371595564631662e-1_f64) * t57147 * t24741 + F::cast_from(0.91464571985215438872e-2_f64) * t83728 * t1808 - F::cast_from(0.11433071498151929859e-2_f64) * t82534 + F::cast_from(0.1219527626469539185e-1_f64) * t82536 - F::cast_from(0.85748036236139473944e-3_f64) * t3718 * t3720 * t90054 * t1250 + F::cast_from(0.25724410870841842184e-2_f64) * t12910 * t3720 * t90059 * t1250 - F::cast_from(0.21240106161011140804e0_f64) * t83296 * t1797;
    (t90037, t90042, t90054, t90059, t90066)
}
