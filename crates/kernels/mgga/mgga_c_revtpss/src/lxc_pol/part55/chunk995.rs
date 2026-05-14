//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 995/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk995<F: Float>(t4147: F, t8713: F, t1450: F, t211: F, t9644: F, t675: F, t886: F, t11006: F, t256: F, t2410: F, t10308: F, t599: F, t90: F, t29: F, t560: F, t9655: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37318 = t4147 * t8713;
    let t38099 = t8713 * t1450;
    let t39643 = 1.0 / t9644 / t211;
    let t41040 = t675 * t886;
    let t41077 = 1.0 / t11006 / t256;
    let t41153 = t2410 * t2410;
    let t41154 = 1.0 / t41153;
    let t45963 = t599 * t10308;
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46361 = 1.0 / t9655 / t560;
    (t37318, t38099, t39643, t41040, t41077, t41154, t45963, t45972, t46361)
}
