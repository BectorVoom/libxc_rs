//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 545/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk545<F: Float>(t265: F, t3821: F, t729: F, t1901: F, t193: F, t3877: F, t3882: F, t3888: F, t3894: F, t3899: F, t3953: F, t3958: F, t3974: F, t3979: F, t3983: F, t3986: F, t3988: F, t3991: F, t3995: F, t446: F, t89: F) -> (F, F) {
    let t3999 = t729 * t265 * t3821;
    let t4002 = t1901 * t3877 / 9.0 + t1901 * t3882 / 9.0 + 2.0 / 9.0 * t1901 * t3888 - 2.0 / 27.0 * t1901 * t3894 + t1901 * t3899 / 9.0 + t89 * t193 * t3953 / 3.0 - t3958 / 9.0 - t446 * t3974 / 3.0 - t446 * t3979 / 3.0 - t446 * t3983 / 3.0 + t3986 / 9.0 + t3988 / 9.0 - t446 * t3991 / 3.0 - t446 * t3995 / 3.0 - t446 * t3999 / 3.0;
    (t3999, t4002)
}
