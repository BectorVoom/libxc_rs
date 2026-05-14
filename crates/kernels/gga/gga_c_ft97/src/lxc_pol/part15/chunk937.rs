//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 937/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk937<F: Float>(t1969: F, t446: F, t86669: F, t62287: F, t62309: F, t62317: F, t78001: F, t78012: F, t78015: F, t78027: F, t87024: F, t87027: F, t87030: F, t87033: F, t87037: F, t87042: F, t87045: F) -> (F, F) {
    let t87048 = t446 * t1969 * t86669;
    let t87050 = -8.0 / 27.0 * t78001 + 4.0 / 27.0 * t78012 - 16.0 / 27.0 * t78015 - 8.0 / 3.0 * t78027 - 8.0 / 27.0 * t62287 - 4.0 / 3.0 * t87024 - 4.0 / 3.0 * t87027 - 8.0 / 9.0 * t87030 - 16.0 / 9.0 * t87033 - t87037 / 3.0 - 16.0 / 81.0 * t62309 + 16.0 / 27.0 * t62317 + 8.0 / 3.0 * t87042 + 4.0 / 9.0 * t87045 + 8.0 / 3.0 * t87048;
    (t87048, t87050)
}
