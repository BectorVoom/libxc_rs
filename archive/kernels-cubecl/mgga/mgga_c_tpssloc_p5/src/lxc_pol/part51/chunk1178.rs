//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1178/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1178<F: Float>(t31160: F, t31177: F, t31157: F, t31163: F, t31166: F, t31173: F, t31179: F, t31576: F, t539: F, t225: F, t567: F, t7191: F) -> (F, F, F, F, F) {
    let t31578 = F::cast_from(0.26915170729426927235e-3_f64) * t31160;
    let t31582 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t31177;
    let t31584 = -t31576 - F::cast_from(0.96894614625936938046e-2_f64) * t31157 - t31578 - F::cast_from(0.16149102437656156341e-2_f64) * t31163 + t31166 / F::cast_from(768.0_f64) - t31173 / F::cast_from(768.0_f64) - t31582 - t31179 / F::cast_from(192.0_f64);
    let t31585 = t539 * t31584;
    let t31589 = t7191 * t225 * t567;
    (t31578, t31582, t31584, t31585, t31589)
}
