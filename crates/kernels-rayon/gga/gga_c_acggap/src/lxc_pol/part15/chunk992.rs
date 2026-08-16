//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 992/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk992(t30798: f64, t30830: f64, t30854: f64, t1432: f64, t1992: f64, t30147: f64, t7586: f64, t30862: f64, t30866: f64, t30874: f64, t30878: f64, t30893: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35004 = 0.21437009059034868486e-3_f64 * t30798;
    let t35012 = 0.20965394859736101379e-2_f64 * t30830;
    let t35018 = 0.25724410870841842184e-2_f64 * t30854;
    let t35022 = t30147 * t7586 * t1992 * t1432;
    let t35024 = 0.12862205435420921092e-1_f64 * t30862;
    let t35025 = 0.17149607247227894789e-2_f64 * t30866;
    let t35028 = 0.32012600194825403606e-1_f64 * t30874;
    let t35030 = 0.16006300097412701803e-1_f64 * t30878;
    let t35034 = 0.28582678745379824648e-3_f64 * t30893;
    (t35004, t35012, t35018, t35022, t35024, t35025, t35028, t35030, t35034)
}
