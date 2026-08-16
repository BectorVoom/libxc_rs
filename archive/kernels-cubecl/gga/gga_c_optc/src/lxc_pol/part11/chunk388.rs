//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 388/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk388<F: Float>(t1974: F, t110: F, t518: F, t84: F, t596: F, t1847: F, t1849: F, t587: F) -> (F, F, F, F) {
    let t1975 = F::cast_from(1.0_f64) / t1974;
    let t1983 = t518 * t110 * t84;
    let t1985 = F::cast_from(0.24415406715670879921e-3_f64) * t596 * t1983;
    let t1990 = t1847 * t1849 * t587;
    (t1975, t1983, t1985, t1990)
}
