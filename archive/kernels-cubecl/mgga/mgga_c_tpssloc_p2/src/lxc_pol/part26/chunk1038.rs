//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1038/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1038<F: Float>(t12237: F, t225: F, t3792: F, t3850: F, t6977: F, t3851: F, t3901: F, t1337: F) -> (F, F, F, F, F) {
    let t12238 = t12237 * t225;
    let t12240 = t3792 * t3850;
    let t12241 = t6977 * t12240;
    let t12244 = t3901 * t3851;
    let t12247 = t1337 * t1337;
    let t12248 = F::cast_from(1.0_f64) / t12247;
    (t12238, t12240, t12241, t12244, t12248)
}
