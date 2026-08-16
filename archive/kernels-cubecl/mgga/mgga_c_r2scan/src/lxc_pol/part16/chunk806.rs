//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 806/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk806<F: Float>(t6535: F, t8139: F, t2834: F, t780: F, t2106: F, t980: F, t6069: F, t7418: F, t2605: F, t6407: F, t2608: F, t6398: F) -> (F, F, F, F, F, F) {
    let t8141 = F::cast_from(0.23287303101564395622e-1_f64) * t6535 * t8139;
    let t8146 = F::cast_from(0.23115257973478049502e0_f64) * t2834 * t780;
    let t8147 = t980 * t2106;
    let t8149 = t6069 * t7418;
    let t8151 = t6407 * t2605;
    let t8153 = t6398 * t2608;
    (t8141, t8146, t8147, t8149, t8151, t8153)
}
