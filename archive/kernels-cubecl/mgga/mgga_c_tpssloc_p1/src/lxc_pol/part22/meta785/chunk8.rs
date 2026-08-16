//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2711/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2711<F: Float>(t16: F, t39031: F, t12774: F, t19503: F, t2: F, t20311: F, t20312: F, t20315: F, t20318: F, t20319: F, t20322: F, t2219: F, t2341: F, t4049: F, t4060: F, t45496: F, t45697: F, t5396: F, t5468: F, t5475: F, t584: F, t657: F, t659: F, t663: F, t75631: F, t92: F, t95: F) -> (F, F) {
    let t75649 = F::cast_from(6.0_f64) * t16 + F::cast_from(12.0_f64) * t39031;
    let t75657 = F::cast_from(50.0_f64) / F::cast_from(81.0_f64) * t657 * t20312 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t92 * t45496 * t20311 * t659 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t45697 * t5468 * t2 * t584 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t657 * t20315 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t45697 * t75631 * t659 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t12774 * t2219 * t5396 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t92 * t4049 * t19503 - F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t657 * t20319 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t92 * t2341 * t20318 * t659 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t92 * t95 * t75649 - F::cast_from(2200.0_f64) / F::cast_from(81.0_f64) * t20322 * t663 + F::cast_from(400.0_f64) / F::cast_from(27.0_f64) * t5475 * t4060;
    (t75649, t75657)
}
