//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2289/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2289(t17611: f64, t6755: f64, t1933: f64, t1934: f64, t5836: f64, t17659: f64, t6765: f64, t1597: f64, t17178: f64, t17183: f64, t18036: f64, t1920: f64, t23437: f64, t23529: f64, t2987: f64, t4509: f64, t5857: f64, t5869: f64, t5909: f64, t6735: f64, t7573: f64, t83016: f64, t83220: f64, t88503: f64, t88517: f64) -> f64 {
    let t99687 = t6755 * t17611;
    let t99692 = t1933 * t1934 * t5836;
    let t99707 = t6765 * t17659;
    let t99709 = -t83220 * t5909 / 216.0_f64 - t23437 * t5869 / 288.0_f64 + t99687 / 2304.0_f64 + t83016 * t18036 / 1152.0_f64 - t88503 + t88517 - 0.10093189023535097714e-3_f64 * t99692 * t6735 - t1920 * t2987 * t17183 / 144.0_f64 + t1920 * t4509 * t17178 / 216.0_f64 - 0.20186378047070195428e-3_f64 * t1933 * t7573 * t1597 * t6735 - t23529 * t5857 / 432.0_f64 + t99707 / 3456.0_f64;
    t99709
}
