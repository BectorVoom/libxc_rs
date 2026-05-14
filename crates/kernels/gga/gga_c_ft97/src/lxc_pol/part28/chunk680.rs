//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 680/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk680<F: Float>(t22563: F, t6: F, t7837: F, t17839: F, t373: F, t58: F, t384: F, t22755: F, t1669: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t32250 = t22563 * t6;
    let t32251 = t7837 * t32250;
    let t32252 = t17839 * sigma0;
    let t32253 = t58 * t373;
    let t32255 = t32252 * t32253 * t384;
    let t32258 = t22755 * t6;
    let t32259 = t1669 * t32258;
    (t32250, t32251, t32252, t32253, t32255, t32258, t32259)
}
