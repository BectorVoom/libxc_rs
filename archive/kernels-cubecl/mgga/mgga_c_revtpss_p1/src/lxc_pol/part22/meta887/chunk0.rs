//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3074/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3074<F: Float>(t1633: F, t3012: F, t2986: F, t4682: F, t11465: F, t1626: F, t15234: F, t3014: F, t11509: F, t4707: F, t11385: F, t1609: F) -> (F, F, F, F, F, F) {
    let t52430 = t3012 * t1633;
    let t52440 = t4682 * t2986;
    let t52443 = t1626 * t11465;
    let t52452 = t15234 * t3014;
    let t52459 = t4707 * t11509;
    let t52482 = t11385 * t1609;
    (t52430, t52440, t52443, t52452, t52459, t52482)
}
