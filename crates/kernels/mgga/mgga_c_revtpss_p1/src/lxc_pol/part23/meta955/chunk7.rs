//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3189/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3189<F: Float>(t20842: F, t5327: F, t17396: F, t20926: F, t12866: F, t58895: F, t6639: F, t1715: F, t5284: F, t17353: F, t17654: F, t17661: F, t17693: F, t1791: F, t20937: F, t21021: F, t21063: F, t21172: F, t3604: F, t44517: F, t5320: F, t58777: F, t59242: F, t6611: F, t70221: F, t70982: F, t71300: F) -> (F, F) {
    let t83748 = t5327 * t20842;
    let t83751 = t17396 * t20926;
    let t83758 = t12866 * t58895 * t6639;
    let t83760 = t1715 * t5284;
    let t83771 = F::cast_from(0.12862205435420921092e-2_f64) * t59242 * t6611 + F::cast_from(0.68598428988911579154e-2_f64) * t70221 * t1791 + F::cast_from(0.68598428988911579154e-2_f64) * t21063 * t5320 - F::cast_from(0.42874018118069736972e-3_f64) * t83748 - F::cast_from(0.11433071498151929859e-2_f64) * t70982 + F::cast_from(0.45732285992607719436e-2_f64) * t83751 - F::cast_from(0.1270341277572436651e-3_f64) * t58777 - F::cast_from(0.85748036236139473944e-3_f64) * t17693 * t71300 * t20937 + F::cast_from(0.57165357490759649296e-3_f64) * t83758 - F::cast_from(0.17149607247227894789e-2_f64) * t17654 * t17353 * t3604 * t83760 - F::cast_from(0.42874018118069736972e-3_f64) * t44517 * t17661 * t21172 + F::cast_from(0.85748036236139473944e-3_f64) * t12866 * t17661 * t21021;
    (t83760, t83771)
}
