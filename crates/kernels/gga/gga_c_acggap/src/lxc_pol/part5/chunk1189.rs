//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1189/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1189<F: Float>(t1008: F, t6361: F, t1163: F, t1166: F, t20417: F, t1096: F, t1165: F, t12401: F, t1426: F, t1531: F, t16314: F, t16707: F, t175: F, t20400: F, t21607: F, t21609: F, t21611: F, t21613: F, t21615: F, t21620: F, t3084: F, t335: F, t336: F, t418: F, t4450: F, t4463: F, t495: F, t5852: F) -> F {
    let t21625 = t1008 * t6361;
    let t21632 = t1163 * t20417 * t1166;
    let t21642 = -F::new(0.42874018118069736972e-3) * t16707 - F::new(0.17149607247227894789e-1) * t4463 * t1165 * t20400 * t1096 - F::new(0.85748036236139473944e-3) * t21607 - F::new(0.42874018118069736972e-3) * t21609 - F::new(0.34299214494455789578e-2) * t21611 + F::new(0.85748036236139473944e-2) * t21613 + F::new(0.85748036236139473944e-2) * t418 * t1426 * t175 * t21615 + F::new(0.42874018118069736972e-2) * t418 * t1426 * t175 * t21620 - F::new(0.51448821741683684366e-1) * t21625 - t335 * t336 * t16314 * t495 / F::new(24.0) + F::new(0.85748036236139473944e-3) * t21632 - F::new(0.12862205435420921092e-2) * t4450 * t1165 * t5852 * t12401 + F::new(0.12862205435420921092e-2) * t1531 * t1165 * t5852 * t3084;
    t21642
}
