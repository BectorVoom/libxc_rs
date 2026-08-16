//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2273/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2273<F: Float>(t5: F, t90107: F, t90135: F, t90167: F, t90199: F, t90230: F, t90265: F, t90315: F, t90346: F, t112: F, t2319: F, t7450: F, t26117: F, t6534: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t90350 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t90107 + t90135 + t90167 + t90199 + t90230 + t90265 + t90315 + t90346);
    let t90351 = t90350 * t112;
    let t90352 = t7450 * t2319;
    let t90355 = F::cast_from(4.0_f64) * t26117 * t6534;
    (t90351, t90352, t90355)
}
