//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 473/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk473<F: Float>(t1023: F, t1058: F, t149: F, t165: F, t4650: F, t4720: F, t4725: F, t4806: F, t4810: F, t4837: F, t4839: F, t184: F) -> (F, F) {
    let t4844 = -F::new(2.0) * t1023 * t1058 - t149 * t4837 - t165 * t4650 - t165 * t4720 + F::new(4.0) * t4725 - F::new(2.0) * t4806 - F::new(4.0) * t4810 + F::new(2.0) * t4839;
    let t4845 = t4844 * t184;
    (t4844, t4845)
}
