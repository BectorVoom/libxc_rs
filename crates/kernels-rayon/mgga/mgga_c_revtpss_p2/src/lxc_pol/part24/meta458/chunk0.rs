//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1428/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1428(t1086: f64, t15669: f64, t3090: f64, t11629: f64, t53703: f64, t3316: f64, t4746: f64, t4891: f64, t1025: f64, t1663: f64, t2434: f64, t371: f64) -> (f64, f64, f64, f64) {
    let t54500 = t15669 * t1086 * t3090;
    let t54564 = t53703 * t11629;
    let t54570 = t4746 * t3316 * t4891;
    let t54687 = t1025 * t371 * t2434 * t1663;
    (t54500, t54564, t54570, t54687)
}
