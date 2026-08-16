//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 403/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk403<F: Float>(t1161: F, t1555: F, t1117: F, t1134: F, t1144: F, t1149: F, t1158: F, t1167: F, t1169: F, t1172: F, t1536: F, t1540: F, t1543: F, t1546: F, t1549: F, t1552: F, t510: F, t518: F) -> (F, F) {
    let t1556 = t1161 * t1555;
    let t1563 = F::cast_from(2.0_f64) * t1117 * t1536 - F::cast_from(2.0_f64) * t510 * t1540 + F::cast_from(6.0_f64) * t1134 * t1543 - F::cast_from(6.0_f64) * t518 * t1546 + F::cast_from(3.0_f64) * t1144 * t1549 - F::cast_from(3.0_f64) * t1149 * t1552 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1158 * t1556 + t1167 * t1549 - t1169 * t1552 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1172 * t1556;
    (t1556, t1563)
}
