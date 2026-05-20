//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2803/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2803<F: Float>(t10529: F, t2782: F, t51529: F, t14602: F, t2482: F, t2811: F, t4423: F, t14575: F, t2435: F, t10943: F, t14598: F, t686: F, t72: F) -> (F, F, F, F) {
    let t51531 = t2782 * t10529 * t51529;
    let t51535 = t2482 * t2811 * t4423 * t14602;
    let t51537 = t2435 * t14575;
    let t51538 = F::cast_from(0.21951497276451705329e-1_f64) * t51537;
    let t51541 = t14598 * t10943 * t72 * t686;
    (t51531, t51535, t51538, t51541)
}
