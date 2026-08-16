//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 197/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk197<F: Float>(t28: F, t517: F, t148: F, t516: F, t157: F, zeta_threshold: F) -> (F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t518 = t517 * t28;
    let t519 = piecewise3::<F>(t29, t148, t518);
    let t520 = t516 + t519 - F::cast_from(2.0_f64);
    let t521 = t520 * t157;
    (t518, t520, t521)
}
