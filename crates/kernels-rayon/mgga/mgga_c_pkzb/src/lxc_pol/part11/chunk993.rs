//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 993/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk993(t10892: f64, t1899: f64, t1084: f64, t9334: f64, t2746: f64, t3551: f64, t3554: f64, t7411: f64, t10867: f64, t1901: f64, t5776: f64, t10769: f64, t10801: f64, t10803: f64, t10807: f64, t10812: f64, t10814: f64, t10816: f64, t10823: f64, t10827: f64, t5783: f64, t5790: f64, t7332: f64, t7357: f64, t9148: f64, t9185: f64, t9192: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10894 = 6.0_f64 * t1899 * t10892;
    let t10896 = 3.0_f64 * t9334 * t1084;
    let t10898 = 3.0_f64 * t2746 * t3551;
    let t10900 = 0.48245938496077605201e2_f64 * t7411 * t3554;
    let t10901 = t10867 * t1901;
    let t10903 = 0.96491876992155210402e2_f64 * t5776 * t10901;
    let t10918 = 0.142419375e1_f64 * t10801 - 0.28483875e1_f64 * t10803 + 0.1898925e1_f64 * t10807 - t5783 + 0.11958666666666666667e1_f64 * t7357 - 0.89690000000000000001e0_f64 * t9148 + 0.8969e0_f64 * t10769 - 0.76790625e-1_f64 * t10812 + 0.46074375e0_f64 * t10814 + 0.3071625e0_f64 * t10816 - t5790 + 0.82156666666666666666e0_f64 * t7332 - 0.49293999999999999999e0_f64 * t9185 - 0.49293999999999999999e0_f64 * t9192 + 0.73941e0_f64 * t10823 + 0.24647e0_f64 * t10827;
    (t10894, t10896, t10898, t10900, t10901, t10903, t10918)
}
