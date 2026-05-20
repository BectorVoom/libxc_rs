//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2642/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2642<F: Float>(t18784: F, t2465: F, t686: F, t72: F, t4481: F, t51276: F, t6042: F, t786: F, t867: F, t2467: F, t14480: F, t252: F, t2782: F, t4533: F) -> (F, F, F, F, F) {
    let t63062 = t2465 * t18784 * t72 * t686;
    let t63064 = t51276 * t4481;
    let t63084 = t786 * t6042 * t867;
    let t63085 = t63084 * t2467;
    let t63091 = t2782 * t252 * t14480 * t4533;
    (t63062, t63064, t63084, t63085, t63091)
}
