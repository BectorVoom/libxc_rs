//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 239/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk239<F: Float>(t799: F, t824: F, t27: F, t89: F, t791: F, t795: F, t788: F) -> (F, F, F, F) {
    let t825 = t799 * t824;
    let t827 = t89 * t27 * t825;
    let t829 = -t791 - t795 / F::cast_from(18.0_f64) - t827 / F::cast_from(6.0_f64);
    let t830 = t788 * t829;
    (t825, t827, t829, t830)
}
