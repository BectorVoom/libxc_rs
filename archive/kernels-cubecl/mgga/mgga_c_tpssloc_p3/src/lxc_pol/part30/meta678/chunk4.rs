//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2124/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2124<F: Float>(t28831: F, t83886: F, t6287: F, t652: F, t6534: F, t26168: F, t7685: F, t19924: F, t24995: F, t8945: F, t19456: F, t7468: F) -> (F, F, F, F, F) {
    let t96755 = F::cast_from(6.0_f64) * t83886 * t28831;
    let t96758 = F::cast_from(2.0_f64) * t652 * t6287 * t6534;
    let t96760 = F::cast_from(6.0_f64) * t7685 * t26168;
    let t96763 = F::cast_from(12.0_f64) * t24995 * t8945 * t19924;
    let t96765 = F::cast_from(4.0_f64) * t19456 * t7468;
    (t96755, t96758, t96760, t96763, t96765)
}
