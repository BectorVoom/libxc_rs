//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 317/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk317<F: Float>(t1010: F, t1011: F, t361: F, t363: F) -> (F, F, F, F) {
    let t1012 = t1010 * t1011;
    let t1013 = t361 * t361;
    let t1014 = F::cast_from(1.0_f64) / t1013;
    let t1015 = t1014 * t363;
    (t1012, t1013, t1014, t1015)
}
