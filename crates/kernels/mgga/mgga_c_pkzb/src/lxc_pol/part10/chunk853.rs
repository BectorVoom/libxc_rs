//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 853/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk853<F: Float>(t2050: F, t735: F, t67: F, t779: F, t154: F, t1885: F, t276: F, t465: F) -> (F, F, F, F) {
    let t5661 = t735 * t2050;
    let t5663 = t67 * t779;
    let t5665 = t154 * t5663 * t1885;
    let t5666 = t276 * t5665;
    let t5672 = t465 * t779;
    (t5661, t5663, t5666, t5672)
}
