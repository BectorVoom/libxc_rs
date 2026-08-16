//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3189/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3189(t20842: f64, t5327: f64, t17396: f64, t20926: f64, t12866: f64, t58895: f64, t6639: f64, t1715: f64, t5284: f64, t17353: f64, t17654: f64, t17661: f64, t17693: f64, t1791: f64, t20937: f64, t21021: f64, t21063: f64, t21172: f64, t3604: f64, t44517: f64, t5320: f64, t58777: f64, t59242: f64, t6611: f64, t70221: f64, t70982: f64, t71300: f64) -> (f64, f64) {
    let t83748 = t5327 * t20842;
    let t83751 = t17396 * t20926;
    let t83758 = t12866 * t58895 * t6639;
    let t83760 = t1715 * t5284;
    let t83771 = 0.12862205435420921092e-2_f64 * t59242 * t6611 + 0.68598428988911579154e-2_f64 * t70221 * t1791 + 0.68598428988911579154e-2_f64 * t21063 * t5320 - 0.42874018118069736972e-3_f64 * t83748 - 0.11433071498151929859e-2_f64 * t70982 + 0.45732285992607719436e-2_f64 * t83751 - 0.1270341277572436651e-3_f64 * t58777 - 0.85748036236139473944e-3_f64 * t17693 * t71300 * t20937 + 0.57165357490759649296e-3_f64 * t83758 - 0.17149607247227894789e-2_f64 * t17654 * t17353 * t3604 * t83760 - 0.42874018118069736972e-3_f64 * t44517 * t17661 * t21172 + 0.85748036236139473944e-3_f64 * t12866 * t17661 * t21021;
    (t83760, t83771)
}
