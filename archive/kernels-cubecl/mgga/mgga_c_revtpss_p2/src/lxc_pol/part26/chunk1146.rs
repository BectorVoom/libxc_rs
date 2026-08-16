//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1146/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1146<F: Float>(t2681: F, t7269: F, t820: F, t1416: F, t25978: F, t3970: F, t240: F, t25981: F, t2661: F, t9935: F, t25987: F, t9775: F) -> (F, F, F, F) {
    let t94545 = t820 * t7269 * t2681;
    let t94546 = t94545 * t1416;
    let t94548 = t25978 * t3970;
    let t94550 = t25981 * t240;
    let t94552 = t2661 * t94550 * t9935;
    let t94554 = t9775 * t25987;
    (t94546, t94548, t94552, t94554)
}
