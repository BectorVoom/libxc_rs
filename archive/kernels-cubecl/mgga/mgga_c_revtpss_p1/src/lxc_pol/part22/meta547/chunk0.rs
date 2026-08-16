//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2362/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2362<F: Float>(t17687: F, t2251: F, t5351: F, t12787: F, t1285: F, t12865: F) -> (F, F, F, F) {
    let t17688 = t17687 * t2251;
    let t17689 = t5351 * t17688;
    let t17690 = t12787 * t17689;
    let t17693 = t1285 * t12865;
    (t17688, t17689, t17690, t17693)
}
