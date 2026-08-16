//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 854/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk854<F: Float>(t10021: F, t241: F, t244: F, t248: F, t238: F, t154: F, t9569: F, t222: F, t2606: F, t9573: F, t119: F, t210: F, t9458: F) -> (F, F, F, F, F, F, F) {
    let t10022 = t10021 * t241;
    let t10024 = t10022 * t244 * t248;
    let t10026 = F::cast_from(595.0_f64) / F::cast_from(10368.0_f64) * t238 * t10024;
    let t10027 = t9569 * t154;
    let t10029 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t10027 * t222;
    let t10030 = t9573 * t2606;
    let t10033 = t210 * t119 * t9458;
    (t10022, t10024, t10026, t10027, t10029, t10030, t10033)
}
