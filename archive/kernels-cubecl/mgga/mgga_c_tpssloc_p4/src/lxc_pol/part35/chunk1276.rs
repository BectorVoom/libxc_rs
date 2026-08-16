//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1276/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1276<F: Float>(t27561: F, t7327: F, t1209: F, t85964: F, t3032: F, t475: F, t210: F, t24810: F, t24848: F, t24594: F, t24847: F, t974: F) -> (F, F, F, F, F) {
    let t86015 = t7327 * t27561;
    let t86022 = t85964 * t1209;
    let t86023 = t3032 * t475;
    let t86036 = t24810 * t210;
    let t86037 = t86036 * t24848;
    let t86076 = t24847 * t974 * t24594;
    (t86015, t86022, t86023, t86037, t86076)
}
