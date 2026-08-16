//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2249/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2249<F: Float>(t23109: F, t23110: F, t232: F, t236: F, t5611: F, t98779: F, t81877: F, t81883: F, t87308: F, t87329: F, t98744: F, t98746: F, t98750: F, t98752: F, t98754: F, t98758: F, t98762: F, t98766: F, t98770: F, t98774: F, t98777: F, t98782: F) -> F {
    let t98787 = t23109 * t23110 * t236 * t5611 * t232;
    let t98791 = t23109 * t23110 * t98779 * t232;
    let t98795 = F::cast_from(0.16956557559538964158e-1_f64) * t98744 + F::cast_from(0.14130464632949136799e-2_f64) * t98746 - F::cast_from(0.16956557559538964159e-1_f64) * t87308 - F::cast_from(0.40372756094140390853e-3_f64) * t98750 + t98752 / F::cast_from(768.0_f64) - t98754 / F::cast_from(768.0_f64) + F::cast_from(0.12111826828242117256e-2_f64) * t98758 - F::cast_from(0.24223653656484234512e-2_f64) * t98762 + F::cast_from(0.80745512188280781708e-3_f64) * t98766 - F::cast_from(0.84782787797694820792e-2_f64) * t98770 - F::cast_from(0.14130464632949136799e-2_f64) * t98774 + t98777 / F::cast_from(1536.0_f64) - F::cast_from(0.6728792682356731809e-4_f64) * t98782 + F::cast_from(0.33643963411783659045e-4_f64) * t98787 + F::cast_from(0.33643963411783659045e-4_f64) * t98791 + F::cast_from(0.16821981705891829522e-4_f64) * t81877 - F::cast_from(0.52708876011794399171e-3_f64) * t81883 - t87329;
    t98795
}
