//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2010/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2010<F: Float>(t3082: F, t6759: F, t344: F, t607: F, t1009: F, t6740: F, t23509: F, t25651: F, t23563: F, t25650: F, t6750: F, t23482: F, t3: F) -> (F, F, F, F, F, F) {
    let t82885 = t6759 * t3082;
    let t82890 = t607 * t344;
    let t82892 = t6740 * t82890 * t1009;
    let t82895 = t23509 * t25651;
    let t82911 = t25650 * t23563;
    let t82914 = t6750 * t3082;
    let t82926 = t23482 * t3;
    (t82885, t82892, t82895, t82911, t82914, t82926)
}
