//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2255/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2255(t23109: f64, t23110: f64, t232: f64, t236: f64, t5611: f64, t98779: f64, t81877: f64, t81883: f64, t87308: f64, t87329: f64, t98744: f64, t98746: f64, t98750: f64, t98752: f64, t98754: f64, t98758: f64, t98762: f64, t98766: f64, t98770: f64, t98774: f64, t98777: f64, t98782: f64) -> f64 {
    let t98787 = t23109 * t23110 * t236 * t5611 * t232;
    let t98791 = t23109 * t23110 * t98779 * t232;
    let t98795 = 0.16956557559538964158e-1_f64 * t98744 + 0.14130464632949136799e-2_f64 * t98746 - 0.16956557559538964159e-1_f64 * t87308 - 0.40372756094140390853e-3_f64 * t98750 + t98752 / 768.0_f64 - t98754 / 768.0_f64 + 0.12111826828242117256e-2_f64 * t98758 - 0.24223653656484234512e-2_f64 * t98762 + 0.80745512188280781708e-3_f64 * t98766 - 0.84782787797694820792e-2_f64 * t98770 - 0.14130464632949136799e-2_f64 * t98774 + t98777 / 1536.0_f64 - 0.6728792682356731809e-4_f64 * t98782 + 0.33643963411783659045e-4_f64 * t98787 + 0.33643963411783659045e-4_f64 * t98791 + 0.16821981705891829522e-4_f64 * t81877 - 0.52708876011794399171e-3_f64 * t81883 - t87329;
    t98795
}
