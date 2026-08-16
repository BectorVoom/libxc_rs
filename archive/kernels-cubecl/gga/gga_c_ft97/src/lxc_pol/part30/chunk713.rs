//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 713/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk713<F: Float>(t7021: F, t840: F, t882: F, t1882: F, t7047: F, t7126: F, t7051: F, t28857: F, t296: F, t7059: F, t2749: F, t7105: F) -> (F, F, F, F, F, F, F) {
    let t29378 = t840 * t882 * t7021;
    let t29383 = t1882 * t7047;
    let t29385 = t1882 * t7126;
    let t29387 = t1882 * t7051;
    let t29389 = t296 * t28857;
    let t29392 = t1882 * t7059;
    let t29396 = t840 * t2749 * t7105;
    (t29378, t29383, t29385, t29387, t29389, t29392, t29396)
}
