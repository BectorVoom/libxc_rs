//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1126/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1126<F: Float>(t22610: F, t22728: F, t56300: F, t56301: F, t56302: F, t56303: F, t56304: F, t56305: F, t56307: F, t56308: F, t56309: F, t16939: F, t3788: F, t4854: F) -> (F, F, F) {
    let t56668 = t56300 - t56301 - t56302 + t22610 - t56303 - t56304 - t56305 + t22728 + t56307 - t56308 - t56309;
    let t56676 = 0.2077890707925103596e3 * t3788 * t16939;
    let t56677 = t4854 * t4854;
    (t56668, t56676, t56677)
}
