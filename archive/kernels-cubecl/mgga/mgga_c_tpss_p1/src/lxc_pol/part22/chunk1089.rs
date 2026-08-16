//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1089/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1089<F: Float>(t11873: F, t11875: F, t4057: F, t664: F) -> (F, F, F) {
    let t11940 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t11873;
    let t11941 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11875;
    let t11942 = t664 * t4057;
    (t11940, t11941, t11942)
}
