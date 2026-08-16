//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1096/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1096<F: Float>(t1073: F, t20045: F, t2265: F, t2266: F, t3613: F, t3621: F, t39514: F, t4454: F, t4458: F, t4872: F, t4883: F, t64926: F, t64969: F, t64985: F, t75996: F, t76199: F, t76210: F, t76267: F, t85456: F, t85465: F, t85474: F, t85491: F, t8654: F, t8680: F, t920: F) -> F {
    let t87906 = F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t64926 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t76199 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t64969 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2265 * t2266 * t75996 * t920 - F::cast_from(12.0_f64) * t2265 * t8680 * t4458 * t4872 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2265 * t8654 * t4454 * t4883 + F::cast_from(6.0_f64) * t2265 * t3621 * t85474 - F::cast_from(2.0_f64) * t2265 * t3613 * t85491 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2265 * t2266 * t20045 * t1073 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2265 * t3621 * t85456 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2265 * t3613 * t85465 - F::cast_from(16.0_f64) * t2265 * t39514 * t76267 * t920 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t76210 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t64985;
    t87906
}
