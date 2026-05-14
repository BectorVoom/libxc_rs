//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 988/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk988<F: Float>(t7021: F, t840: F, t882: F, t1882: F, t7047: F, t7126: F, t7051: F, t28857: F, t296: F, t7059: F, t2749: F, t7105: F, t1212: F, t6386: F, t871: F, t681: F, t7093: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29378 = t840 * t882 * t7021;
    let t29383 = t1882 * t7047;
    let t29385 = t1882 * t7126;
    let t29387 = t1882 * t7051;
    let t29389 = t296 * t28857;
    let t29392 = t1882 * t7059;
    let t29396 = t840 * t2749 * t7105;
    let t29399 = t6386 * t1212;
    let t29401 = t840 * t871 * t29399;
    let t29405 = t89 * t681 * t7093;
    (t29378, t29383, t29385, t29387, t29389, t29392, t29396, t29399, t29401, t29405)
}
