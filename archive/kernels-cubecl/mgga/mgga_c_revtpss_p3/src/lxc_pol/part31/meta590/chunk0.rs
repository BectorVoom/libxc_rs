//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2014/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2014<F: Float>(t25240: F, t3951: F, t3964: F, t2681: F, t7269: F, t820: F, t1416: F, t240: F, t25981: F, t25987: F, t9775: F, t2453: F, t4086: F, t64: F) -> (F, F, F, F, F, F) {
    let t94540 = t3964 * t25240 * t3951;
    let t94545 = t820 * t7269 * t2681;
    let t94546 = t94545 * t1416;
    let t94550 = t25981 * t240;
    let t94554 = t9775 * t25987;
    let t94564 = t2453 * t4086 * t64;
    (t94540, t94545, t94546, t94550, t94554, t94564)
}
