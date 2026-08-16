//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2955/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2955<F: Float>(t11862: F, t11875: F, t11991: F, t12017: F, t15926: F, t3117: F, t3157: F, t3162: F, t42391: F, t42487: F, t42496: F, t4803: F, t4875: F, t53790: F, t53792: F, t53800: F, t53805: F, t53807: F, t53810: F) -> F {
    let t53816 = -F::cast_from(0.57165357490759649295e-3_f64) * t53790 + F::cast_from(0.64311027177104605458e-3_f64) * t11875 * t3117 * t53792 * t3162 - F::cast_from(0.64311027177104605458e-3_f64) * t15926 * t12017 - F::cast_from(0.12862205435420921092e-2_f64) * t53800 * t11862 + F::cast_from(0.85748036236139473944e-3_f64) * t42487 + F::cast_from(0.95275595817932748827e-4_f64) * t42496 - F::cast_from(0.57165357490759649295e-3_f64) * t53805 - F::cast_from(0.68598428988911579154e-2_f64) * t53807 * t3157 + F::cast_from(0.85748036236139473944e-3_f64) * t53810 - F::cast_from(0.42874018118069736972e-3_f64) * t42391 * t4875 - F::cast_from(0.85748036236139473944e-3_f64) * t11991 * t4803;
    t53816
}
