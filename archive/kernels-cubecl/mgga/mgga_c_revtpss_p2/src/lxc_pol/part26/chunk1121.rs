//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1121/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1121<F: Float>(t10671: F, t7033: F, t25255: F, t2689: F, t10680: F, t1945: F, t807: F, t10690: F, t9646: F, t10674: F, t7030: F, t9789: F) -> (F, F, F, F, F, F) {
    let t92999 = t7033 * t10671;
    let t93001 = t2689 * t25255;
    let t93004 = t807 * t1945 * t10680;
    let t93007 = t9646 * t1945 * t10690;
    let t93010 = t807 * t1945 * t10674;
    let t93012 = t9789 * t7030;
    (t92999, t93001, t93004, t93007, t93010, t93012)
}
