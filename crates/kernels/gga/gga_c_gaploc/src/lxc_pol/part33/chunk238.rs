//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 238/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk238<F: Float>(t105: F, t877: F, t886: F, t889: F, t189: F, t874: F) -> (F, F) {
    let t892 = 0.28455006635676149599e-1 * t105 * t877 + 0.11856252764865062333e-2 * t886 - 0.28455006635676149599e-1 * t105 * t889;
    let t894 = t189 * t874;
    (t892, t894)
}
