//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2025/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2025<F: Float>(t11553: F, t2121: F, t2148: F, t27561: F, t7327: F, t210: F, t24810: F, t24848: F, t1090: F, t24815: F, t24594: F, t24847: F, t974: F) -> (F, F, F, F, F, F) {
    let t86000 = F::cast_from(0.30461741978670859935e-2_f64) * t2121 * t11553 * t2148;
    let t86015 = t7327 * t27561;
    let t86036 = t24810 * t210;
    let t86037 = t86036 * t24848;
    let t86039 = t24815 * t1090;
    let t86076 = t24847 * t974 * t24594;
    (t86000, t86015, t86036, t86037, t86039, t86076)
}
