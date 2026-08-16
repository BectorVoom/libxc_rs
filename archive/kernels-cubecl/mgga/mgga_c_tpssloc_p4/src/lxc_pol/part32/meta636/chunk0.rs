//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2050/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2050<F: Float>(t87535: F, t23185: F, t4283: F, t81914: F, t25300: F, t81591: F, t25303: F, t6579: F, t23110: F, t4292: F, t25288: F, t234: F, t4265: F) -> (F, F, F, F, F, F, F) {
    let t87536 = F::cast_from(0.38381794893125283518e-1_f64) * t87535;
    let t87544 = t23185 * t81914 * t4283;
    let t87545 = F::cast_from(0.16449340668482264365e-1_f64) * t87544;
    let t87546 = t81591 * t25300;
    let t87547 = F::cast_from(0.76763589786250567036e-1_f64) * t87546;
    let t87565 = t6579 * t25303;
    let t87566 = F::cast_from(0.76763589786250567036e-1_f64) * t87565;
    let t87581 = t23185 * t23110 * t4292;
    let t87582 = F::cast_from(0.82246703342411321824e-2_f64) * t87581;
    let t87583 = t81591 * t25288;
    let t87584 = F::cast_from(0.76763589786250567036e-1_f64) * t87583;
    let t87586 = t234 * t4265;
    (t87536, t87545, t87547, t87566, t87582, t87584, t87586)
}
