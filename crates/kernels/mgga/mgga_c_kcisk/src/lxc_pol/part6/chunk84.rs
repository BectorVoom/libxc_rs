//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 84/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk84<F: Float>(t222: F, t5: F, t7: F, tau0: F, zeta_threshold: F) -> (F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t295 = F::new(1.0) / tau0;
    let t296 = piecewise3::<f64>(t223, zeta_threshold, t222);
    let t297 = t295 * t296;
    let t298 = t5 * t7;
    (t295, t297, t298)
}
