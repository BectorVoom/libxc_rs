//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1938/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1938<F: Float>(t100: F, t580: F, t22: F, t4273: F, t10241: F, t1509: F, t2358: F, t105: F, t2357: F, t2255: F, t661: F, t2362: F, t4279: F) -> (F, F, F, F, F, F) {
    let t13482 = t100 * t580;
    let t13485 = t4273 * t22;
    let t13493 = t10241 * t1509 * t2358;
    let t13496 = t105 * t2357;
    let t13497 = t2255 * t661;
    let t13500 = t4279 * t2362;
    (t13482, t13485, t13493, t13496, t13497, t13500)
}
