//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1814/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1814<F: Float>(t276: F, t285: F, t273: F, t2439: F, t931: F) -> (F, F, F) {
    let t11354 = F::new(1.0) / t276 / t285 / F::new(4.0);
    let t11358 = F::new(1.0)/pow_3_2::<F>(t273);
    let t11366 = t2439 * t931;
    (t11354, t11358, t11366)
}
