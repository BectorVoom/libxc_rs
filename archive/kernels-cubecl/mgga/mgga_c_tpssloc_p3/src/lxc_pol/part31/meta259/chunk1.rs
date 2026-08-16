//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1083/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1083<F: Float>(t7101: F, t829: F, t235: F, t7084: F, t2051: F, t226: F, t6641: F, t6650: F, t6654: F, t7095: F, t7097: F, t808: F, t812: F) -> (F, F, F) {
    let t7102 = t7101 * t829;
    let t7104 = t235 * t7084;
    let t7106 = -t7095 - F::cast_from(0.3289868133696452873e-1_f64) * t6641 - t7097 - F::cast_from(0.16449340668482264365e-1_f64) * t6650 + F::cast_from(0.16449340668482264365e-1_f64) * t6654 + t808 * t2051 - t812 * t7102 + t226 * t7104;
    (t7102, t7104, t7106)
}
