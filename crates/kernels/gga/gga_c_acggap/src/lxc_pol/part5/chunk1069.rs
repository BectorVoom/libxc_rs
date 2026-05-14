//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1069/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1069<F: Float>(t1096: F, t1165: F, t12401: F, t1426: F, t1531: F, t16314: F, t16707: F, t175: F, t20400: F, t21607: F, t21609: F, t21611: F, t21613: F, t21615: F, t21620: F, t21625: F, t21632: F, t3084: F, t335: F, t336: F, t418: F, t4450: F, t4463: F, t495: F, t5852: F) -> (F,) {
    let t21642 = -0.42874018118069736972e-3 * t16707 - 0.17149607247227894789e-1 * t4463 * t1165 * t20400 * t1096 - 0.85748036236139473944e-3 * t21607 - 0.42874018118069736972e-3 * t21609 - 0.34299214494455789578e-2 * t21611 + 0.85748036236139473944e-2 * t21613 + 0.85748036236139473944e-2 * t418 * t1426 * t175 * t21615 + 0.42874018118069736972e-2 * t418 * t1426 * t175 * t21620 - 0.51448821741683684366e-1 * t21625 - t335 * t336 * t16314 * t495 / 24.0 + 0.85748036236139473944e-3 * t21632 - 0.12862205435420921092e-2 * t4450 * t1165 * t5852 * t12401 + 0.12862205435420921092e-2 * t1531 * t1165 * t5852 * t3084;
    (t21642,)
}
