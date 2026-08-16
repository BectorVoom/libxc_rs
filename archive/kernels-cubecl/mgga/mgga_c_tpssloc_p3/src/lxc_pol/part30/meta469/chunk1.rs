//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1757/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1757<F: Float>(t1004: F, t6764: F, t1036: F, t6750: F, t6759: F, t3: F, t6740: F) -> (F, F, F, F) {
    let t23544 = t1004 * t6764;
    let t23554 = t6750 * t1036;
    let t23560 = t6759 * t1036;
    let t23562 = t6740 * t3;
    (t23544, t23554, t23560, t23562)
}
