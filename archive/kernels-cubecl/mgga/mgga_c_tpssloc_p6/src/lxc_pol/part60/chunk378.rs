//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 378/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk378<F: Float>(t2369: F, t2509: F, t2512: F, t761: F, t2504: F, t739: F, t746: F, t15: F, t60: F, t59: F, t207: F, t215: F) -> (F, F, F, F, F, F, F) {
    let t2527 = t2509 * t2369;
    let t2528 = t2527 * t2512;
    let t2530 = F::cast_from(0.17315859105681463759e2_f64) * t761 * t2528;
    let t2535 = t739 * t2504 * t746;
    let t2537 = F::cast_from(0.5848223622634646207e0_f64) * t761 * t2535;
    let t2558 = F::cast_from(1.0_f64) / t60 / t15;
    let t2559 = t59 * t2558;
    let t2562 = F::cast_from(0.64814814814814814813e-2_f64) * t2559 * t207 * t215;
    (t2528, t2530, t2535, t2537, t2558, t2559, t2562)
}
