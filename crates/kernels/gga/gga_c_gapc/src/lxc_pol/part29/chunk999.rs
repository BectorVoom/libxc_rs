//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 999/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk999<F: Float>(t3707: F, t4780: F, t2660: F, t7375: F, t33312: F, t3789: F, t11449: F, t11804: F, t15843: F, t190: F, t2674: F, t11522: F, t15805: F, t9799: F, t34104: F, t34108: F, t34111: F, t34114: F, t34117: F, t34119: F, t34121: F) -> (F, F) {
    let t34123 = t4780 * t3707;
    let t34125 = t2660 * t34123 * t7375;
    let t34127 = t33312 * t3789;
    let t34132 = t2674 * t190 * t11449 * t11804 * t15843;
    let t34135 = t15805 * t11522 * t9799;
    let t34137 = 0.2318836277704281739e-4 * t34104 + 0.56360603971979070047e-7 * t34108 + 0.34752370105806885418e-3 * t34111 - 0.24581606547037760418e-7 * t34114 + 0.12290803273518880209e-8 * t34117 - 0.35170937063461460536e-8 * t34119 - 0.35170937063461460536e-8 * t34121 + 0.4797801045921060808e-7 * t34125 + 0.17089546493091976008e-5 * t34127 - 0.12290803273518880209e-8 * t34132 + 0.12650553385416666667e-5 * t34135;
    (t34123, t34137)
}
