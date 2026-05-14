//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 933/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk933<F: Float>(t446: F, t86977: F, t9049: F, t40599: F, t61462: F, t62134: F, t86608: F, t86937: F, t86942: F, t86946: F, t86950: F, t86954: F, t86958: F, t86962: F, t86966: F, t86970: F, t86975: F) -> (F, F) {
    let t86979 = t446 * t9049 * t86977;
    let t86981 = -5.0 / 16.0 * t86608 + t86937 / 6.0 + 16.0 / 27.0 * t61462 + t40599 - 12.0 * t86942 + 8.0 / 3.0 * t86946 - 80.0 / 243.0 * t86950 - 8.0 / 3.0 * t86954 - t86958 / 9.0 + 8.0 / 3.0 * t86962 + 2.0 / 3.0 * t86966 - 2.0 / 9.0 * t86970 + 16.0 / 9.0 * t62134 - 8.0 / 3.0 * t86975 + 8.0 / 9.0 * t86979;
    (t86979, t86981)
}
