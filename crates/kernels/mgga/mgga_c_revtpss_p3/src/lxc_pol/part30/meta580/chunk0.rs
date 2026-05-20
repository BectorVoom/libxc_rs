//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2033/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2033<F: Float>(t25240: F, t3951: F, t3964: F, t25972: F, t9761: F, t2681: F, t7269: F, t820: F, t1416: F, t25978: F, t3970: F, t240: F, t25981: F) -> (F, F, F, F, F, F) {
    let t94540 = t3964 * t25240 * t3951;
    let t94542 = t25972 * t9761;
    let t94545 = t820 * t7269 * t2681;
    let t94546 = t94545 * t1416;
    let t94548 = t25978 * t3970;
    let t94550 = t25981 * t240;
    (t94540, t94542, t94545, t94546, t94548, t94550)
}
