//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1333/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1333<F: Float>(t10186: F, t10196: F, t10241: F, t10246: F, t10255: F, t10259: F, t10260: F, t2986: F, t2990: F, t346: F, t42759: F, t42762: F, t42773: F, t42775: F, t42785: F, t42788: F) -> F {
    let t42790 = F::cast_from(0.21547325102880658436e0_f64) * t42759 * t346 - F::cast_from(0.50699588477366255142e-1_f64) * t42762 - F::cast_from(0.11851851851851851852e-1_f64) * t10186 * t10196 + F::cast_from(0.33333333333333333332e-2_f64) * t2986 * t10259 * t10255 + F::cast_from(0.88888888888888888887e-2_f64) * t10186 * t10260 - F::cast_from(0.11111111111111111111e-2_f64) * t42773 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t42775 * t2990 + F::cast_from(0.33333333333333333332e-2_f64) * t2986 * t10241 * t10255 + F::cast_from(0.88888888888888888886e-2_f64) * t10186 * t10246 - F::cast_from(0.11111111111111111111e-2_f64) * t42785 - F::cast_from(0.22222222222222222222e-2_f64) * t42788;
    t42790
}
