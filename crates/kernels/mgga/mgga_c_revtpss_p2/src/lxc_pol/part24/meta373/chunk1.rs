//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1265/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1265<F: Float>(t1715: F, t21093: F, t1042: F, t1774: F, t5819: F, t5268: F, t6573: F) -> (F, F, F, F, F, F) {
    let t24604 = t21093 * t1715;
    let t24605 = t1042 * t24604;
    let t24610 = t5819 * t1774;
    let t24611 = t5268 * t24610;
    let t24612 = t1042 * t24611;
    let t24616 = t6573 * t1774;
    (t24604, t24605, t24610, t24611, t24612, t24616)
}
