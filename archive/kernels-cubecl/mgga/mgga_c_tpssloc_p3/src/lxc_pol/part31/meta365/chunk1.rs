//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1291/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1291<F: Float>(t16586: F, t2658: F, t2523: F, t5527: F, t262: F, t5544: F, t1484: F, t868: F) -> (F, F, F, F) {
    let t16587 = t2658 * t16586;
    let t16588 = F::cast_from(12.0_f64) * t16587;
    let t16589 = t2523 * t5527;
    let t16592 = t262 * t5544;
    let t16596 = t1484 * t868;
    (t16588, t16589, t16592, t16596)
}
