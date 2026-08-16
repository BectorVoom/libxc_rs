//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 832/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk832(t509: f64, t5935: f64, t1270: f64, t1845: f64, t5757: f64, t1163: f64, t118: f64, t1273: f64, t1760: f64, t1796: f64, t1800: f64, t1830: f64, t1834: f64, t1846: f64, t2056: f64, t3499: f64, t485: f64, t544: f64, t5706: f64, t5799: f64, t5801: f64, t5809: f64, t5816: f64, t5820: f64, t5895: f64, t5905: f64, t5910: f64, t624: f64, t626: f64, t646: f64) -> (f64, f64, f64, f64) {
    let t5936 = t509 * t5935;
    let t5937 = t5936 * t1270;
    let t5939 = t1845 * t5757;
    let t5941 = -t1163 * t1796 - t118 * t5895 + t1273 * t1834 + 3.0_f64 * t1760 * t5910 + t1760 * t5937 - t1760 * t5939 - 2.0_f64 * t1800 * t2056 - 2.0_f64 * t1800 * t3499 - t1830 * t624 + t1846 * t5706 - t485 * t5799 + t544 * t5905 - 2.0_f64 * t5801 * t646 - 2.0_f64 * t5809 * t626 - 2.0_f64 * t5816 * t626 - 2.0_f64 * t5820 * t626;
    (t5936, t5937, t5939, t5941)
}
