//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 976/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk976<F: Float>(t17912: F, t2288: F, t31443: F, t3169: F, t1588: F, t7614: F, t1988: F, t8855: F, t7799: F, t8859: F, t422: F, t4875: F, t598: F, t599: F, t6: F, t1488: F, t1980: F, t1982: F, t1983: F) -> (F, F, F, F, F, F) {
    let t35808 = t31443 * t17912 * t2288 * t3169;
    let t35814 = t7614 * t1588;
    let t35816 = t1988 * t8855;
    let t35818 = t7799 * t8859;
    let t35823 = t598 * t422 * t6 * t4875 * t599;
    let t35827 = t1980 * t1982 * t1488 * t1983;
    (t35808, t35814, t35816, t35818, t35823, t35827)
}
