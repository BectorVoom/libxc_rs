//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 753/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk753<F: Float>(t1642: F, t379: F, t32140: F, t32261: F, t7195: F, t5517: F, t78: F, t5560: F, t7853: F, t1602: F, t5544: F, t17839: F, t58: F) -> (F, F, F, F, F, F, F) {
    let t32280 = t1642 * t379;
    let t32281 = t32140 * t32280;
    let t32284 = t7195 * t32261;
    let t32289 = t5517 * t78;
    let t32292 = t7853 * t5560;
    let t32295 = t1602 * t5544;
    let t32296 = t17839 * t58;
    (t32280, t32281, t32284, t32289, t32292, t32295, t32296)
}
