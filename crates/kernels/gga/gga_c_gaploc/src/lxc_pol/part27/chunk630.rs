//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 630/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk630<F: Float>(t1953: F, t747: F, t1959: F, t744: F, t746: F, t304: F) -> (F, F, F, F, F) {
    let t5549 = t1953 * t747;
    let t5552 = t744 * t1959;
    let t5557 = t746 * t746;
    let t5558 = 1.0 / t5557;
    let t5559 = t304 * t5558;
    (t5549, t5552, t5557, t5558, t5559)
}
