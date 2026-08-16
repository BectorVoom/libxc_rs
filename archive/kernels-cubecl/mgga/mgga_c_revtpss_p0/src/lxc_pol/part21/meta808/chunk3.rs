//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2950/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2950<F: Float>(t1042: F, t1045: F, t1063: F, t11202: F, t11252: F, t11259: F, t11933: F, t1469: F, t15716: F, t16045: F, t3115: F, t3117: F, t3130: F, t42421: F, t42439: F, t4872: F, t51963: F, t53474: F, t53683: F, t53690: F, t53692: F, t53704: F, t53707: F, t53710: F) -> F {
    let t53716 = -F::cast_from(0.21437009059034868486e-3_f64) * t3115 * t3117 * t53683 * t1045 - F::cast_from(0.30488190661738479624e-2_f64) * t42421 + F::cast_from(0.57165357490759649295e-3_f64) * t53690 - F::cast_from(0.85748036236139473944e-3_f64) * t53692 * t3130 - F::cast_from(0.85748036236139473944e-3_f64) * t15716 * t1042 * t4872 * t1469 * t11202 - F::cast_from(0.57165357490759649295e-3_f64) * t42439 + F::cast_from(0.34299214494455789577e-2_f64) * t11933 * t16045 - F::cast_from(0.12862205435420921092e-2_f64) * t53704 * t11252 + F::cast_from(0.21437009059034868486e-3_f64) * t53707 * t11259 - F::cast_from(0.57165357490759649295e-3_f64) * t53710 + F::cast_from(0.85748036236139473944e-2_f64) * t1063 * t1042 * t51963 * t53474;
    t53716
}
