//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 925/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk925<F: Float>(t50: F, t17463: F, t17548: F, t17659: F, t18223: F, t16241: F, t1438: F, t1440: F, t1591: F, t1593: F, t17322: F, t17329: F, t208: F, t367: F, t368: F, t501: F, t502: F, t5080: F, t5084: F, t5479: F, t5483: F, zeta_threshold: F) -> (F, F, F) {
    let t51 = t50 <= zeta_threshold;
    let t18225 = t17463 + t17548 + t17659 + t18223;
    let t18232 = piecewise3(t51, 0.0, t16241);
    let t18236 = t208 * (t17322 * t368 / 2.0 + 3.0 / 2.0 * t5080 * t1440 + 3.0 / 2.0 * t1438 * t5084 + t367 * t17329 / 2.0 + t18225 * t502 / 2.0 + 3.0 / 2.0 * t5479 * t1593 + 3.0 / 2.0 * t1591 * t5483 + t501 * t18232 / 2.0);
    (t18225, t18232, t18236)
}
