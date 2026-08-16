//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1742/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1742(t5819: f64, t6573: f64, t6587: f64, t6628: f64, t1794: f64, t24633: f64, t6622: f64, t1042: f64, t1250: f64, t12809: f64, t12910: f64, t1797: f64, t1808: f64, t24741: f64, t3611: f64, t3718: f64, t3720: f64, t5302: f64, t5384: f64, t57147: f64, t82469: f64, t82491: f64, t82534: f64, t82536: f64, t83296: f64, t83728: f64) -> (f64, f64, f64, f64, f64) {
    let t90037 = t5819 * t6573;
    let t90042 = t6587 * t6628;
    let t90054 = t24633 * t1794;
    let t90059 = t6573 * t6622;
    let t90066 = 0.34299214494455789578e-2_f64 * t82469 + 0.28582678745379824648e-2_f64 * t5384 * t1042 * t5302 * t90037 + 0.12862205435420921092e-2_f64 * t12809 * t3720 * t90042 * t3611 + 0.34299214494455789578e-2_f64 * t82491 - 0.27439371595564631662e-1_f64 * t57147 * t24741 + 0.91464571985215438872e-2_f64 * t83728 * t1808 - 0.11433071498151929859e-2_f64 * t82534 + 0.1219527626469539185e-1_f64 * t82536 - 0.85748036236139473944e-3_f64 * t3718 * t3720 * t90054 * t1250 + 0.25724410870841842184e-2_f64 * t12910 * t3720 * t90059 * t1250 - 0.21240106161011140804e0_f64 * t83296 * t1797;
    (t90037, t90042, t90054, t90059, t90066)
}
