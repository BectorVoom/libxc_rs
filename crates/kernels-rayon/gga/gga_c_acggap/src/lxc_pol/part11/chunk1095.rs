//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1095/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1095(t1432: f64, t1992: f64, t30147: f64, t7586: f64, t30862: f64, t30866: f64, t30874: f64, t30878: f64, t30893: f64, t30868: f64, t30872: f64, t30876: f64, t30880: f64, t30883: f64, t30884: f64, t30887: f64, t30890: f64, t30891: f64, t30901: f64, t30905: f64, t30908: f64) -> f64 {
    let t35022 = t30147 * t7586 * t1992 * t1432;
    let t35024 = 0.12862205435420921092e-1_f64 * t30862;
    let t35025 = 0.17149607247227894789e-2_f64 * t30866;
    let t35028 = 0.32012600194825403606e-1_f64 * t30874;
    let t35030 = 0.16006300097412701803e-1_f64 * t30878;
    let t35034 = 0.28582678745379824648e-3_f64 * t30893;
    let t35036 = 0.28582678745379824648e-3_f64 * t35022 - t35024 - t35025 + 0.45351183609335988443e-1_f64 * t30868 - 0.45351183609335988443e-1_f64 * t30872 + t35028 + 0.80031500487063509016e-2_f64 * t30876 - t35030 + 0.90035438047946447642e-2_f64 * t30880 + t30883 - 0.40015750243531754508e-1_f64 * t30884 - t30887 - t30890 + 0.10718504529517434243e-3_f64 * t30891 + t35034 + 0.7145669686344956162e-4_f64 * t30901 - t30905 - t30908;
    t35036
}
