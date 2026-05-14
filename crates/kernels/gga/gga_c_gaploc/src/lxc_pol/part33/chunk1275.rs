//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1275/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1275<F: Float>(t28987: F, t28990: F, t33853: F, t33857: F, t33859: F, t33861: F, t33863: F, t33865: F, t33867: F, t33869: F, t33872: F, t33878: F, t33881: F, t33883: F, t33891: F, t33892: F) -> (F,) {
    let t39302 = -t33853 - t33857 - t33859 - t33861 + t33863 + t33865 + t33867 + t33869 - t33872 + t33878 + t33881 + t33883 - t33891 - 0.10224780254378866581e1 * t28987 + 0.53964118009221795842e0 * t28990 - t33892;
    (t39302,)
}
