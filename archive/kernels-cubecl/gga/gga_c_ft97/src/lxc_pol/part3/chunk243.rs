//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 243/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk243<F: Float>(t799: F, t824: F, t27: F, t89: F, t791: F, t795: F, t788: F, t313: F, t681: F, t295: F, t683: F) -> (F, F, F, F, F) {
    let t825 = t799 * t824;
    let t827 = t89 * t27 * t825;
    let t829 = -t791 - t795 / F::cast_from(18.0_f64) - t827 / F::cast_from(6.0_f64);
    let t830 = t788 * t829;
    let t834 = t89 * t681 * t313 / F::cast_from(9.0_f64);
    let t835 = t683 * t295;
    (t825, t827, t830, t834, t835)
}
