//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 863/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk863<F: Float>(t31569: F, t6897: F, t1323: F, t8617: F, t31153: F, t31160: F, t31177: F, t31157: F, t31163: F, t31166: F, t31173: F, t31179: F) -> (F, F, F) {
    let t31570 = t6897 * t31569;
    let t31571 = F::cast_from(0.41123351671205660912e-2_f64) * t31570;
    let t31573 = t1323 * t8617;
    let t31576 = F::cast_from(0.11304371706359309439e-1_f64) * t31153;
    let t31578 = F::cast_from(0.26915170729426927235e-3_f64) * t31160;
    let t31582 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t31177;
    let t31584 = -t31576 - F::cast_from(0.96894614625936938046e-2_f64) * t31157 - t31578 - F::cast_from(0.16149102437656156341e-2_f64) * t31163 + t31166 / F::cast_from(768.0_f64) - t31173 / F::cast_from(768.0_f64) - t31582 - t31179 / F::cast_from(192.0_f64);
    (t31571, t31573, t31584)
}
