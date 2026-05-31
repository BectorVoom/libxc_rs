//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 265/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk265<F: Float>(t317: F, t502: F, t313: F, t807: F, t811: F) -> (F, F) {
    let t825 = t317 * t502 / F::cast_from(3.0_f64);
    let t826 = F::cast_from(3.0_f64) / F::cast_from(10.0_f64) * t313 * (F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t807 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t811) - t825;
    (t825, t826)
}
