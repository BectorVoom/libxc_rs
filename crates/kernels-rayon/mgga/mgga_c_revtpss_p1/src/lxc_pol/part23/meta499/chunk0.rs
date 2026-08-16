//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1983/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1983(t1211: f64, t20747: f64, t487: f64, t6564: f64, t1770: f64, t1811: f64, t1294: f64, t6744: f64, t3737: f64, t1248: f64, t1715: f64, t3604: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20748 = t1211 * t20747;
    let t20753 = t6564 * t487;
    let t20756 = t1770 * t1811;
    let t20759 = t6744 * t1294;
    let t20760 = t3737 * t20759;
    let t20765 = t1715 * t1248;
    let t20766 = t3604 * t20765;
    (t20748, t20753, t20756, t20759, t20760, t20765, t20766)
}
