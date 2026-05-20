//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 997/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk997<F: Float>(t13955: F, t9845: F, t1885: F, t9909: F, t4000: F, t820: F, t844: F, t2713: F, t3964: F, t5617: F, t5665: F, t9976: F) -> (F, F, F, F, F) {
    let t13956 = t9845 * t13955;
    let t13959 = t9909 * t1885;
    let t13999 = t820 * t4000 * t844;
    let t14013 = t3964 * t2713 * t5617;
    let t14043 = t9976 * t5665;
    (t13956, t13959, t13999, t14013, t14043)
}
