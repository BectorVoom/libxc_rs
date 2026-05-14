//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1085/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1085<F: Float>(t11514: F, t5633: F, t137: F, t1743: F, t190: F, t33235: F, t442: F, t5971: F, t11484: F, t1835: F, t1691: F, t1040: F, t34382: F, t11387: F, t2993: F, t8793: F) -> (F, F, F, F, F, F) {
    let t34995 = t11514 * t5633;
    let t35001 = t1743 * t33235 * t5971 * t190 * t137 * t442;
    let t35003 = t11484 * t1835;
    let t35005 = t11484 * t1691;
    let t35007 = t34382 * t1040;
    let t35010 = t2993 * t11387 * t8793;
    (t34995, t35001, t35003, t35005, t35007, t35010)
}
