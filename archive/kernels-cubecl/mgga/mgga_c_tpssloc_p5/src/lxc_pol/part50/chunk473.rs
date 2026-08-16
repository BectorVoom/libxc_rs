//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 473/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk473<F: Float>(t868: F, t870: F, t2369: F, t2509: F, t2512: F, t761: F, t172: F, t753: F, t763: F, t2504: F, t739: F, t746: F) -> (F, F, F, F, F) {
    let t2523 = t868 * t870;
    let t2527 = t2509 * t2369;
    let t2528 = t2527 * t2512;
    let t2530 = F::cast_from(0.17315859105681463759e2_f64) * t761 * t2528;
    let t2531 = t753 * t172;
    let t2532 = t2531 * t763;
    let t2535 = t739 * t2504 * t746;
    (t2523, t2528, t2530, t2532, t2535)
}
