//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1295/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1295<F: Float>(t17624: F, t6717: F, t1933: F, t1937: F, t5398: F, t40: F, t5842: F, t23479: F, t1409: F, t1597: F, t23562: F, t343: F) -> (F, F, F, F, F, F) {
    let t99624 = t6717 * t17624;
    let t99631 = t1933 * t5398 * t1937;
    let t99645 = t40 * t5842;
    let t99647 = t1933 * t99645 * t23479;
    let t99660 = t1409 * t1597;
    let t99662 = t23562 * t99660 * t343;
    (t99624, t99631, t99645, t99647, t99660, t99662)
}
