//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 979/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk979<F: Float>(t3753: F, t530: F, t174: F, t1331: F, t2331: F, t251: F, t3977: F, t11407: F, t250: F, t3106: F, t461: F, t453: F) -> (F, F, F, F, F, F, F, F) {
    let t11418 = F::new(1.0) / t3753 / t530;
    let t11425 = F::new(1.0) / t3753 / t174;
    let t11455 = t2331 * t1331;
    let t11462 = t251 * t3977;
    let t11479 = F::cast_from(0.93932222222222222223e0_f64) * t11407;
    let t11481 = t250 * t3106 * t461;
    let t11482 = F::cast_from(0.36793333333333333333e0_f64) * t11481;
    let t11491 = F::new(1.0)/pow_3_2::<F>(t453);
    (t11418, t11425, t11455, t11462, t11479, t11481, t11482, t11491)
}
