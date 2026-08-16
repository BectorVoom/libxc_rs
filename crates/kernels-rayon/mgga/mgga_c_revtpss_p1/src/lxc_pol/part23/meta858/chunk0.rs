//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2748/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2748(t3603: f64, t43350: f64, t13126: f64, t1811: f64, t460: f64, t3566: f64, t6695: f64, t5216: f64, t17288: f64, t488: f64, t5219: f64, t487: f64, t69636: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t72724 = t43350 * t3603;
    let t72732 = t460 * t13126 * t1811;
    let t72767 = t3566 * t6695;
    let t72784 = t5216 * t1811;
    let t72787 = t17288 * t1811;
    let t72794 = t5219 * t488;
    let t72802 = t69636 * t487;
    (t72724, t72732, t72767, t72784, t72787, t72794, t72802)
}
