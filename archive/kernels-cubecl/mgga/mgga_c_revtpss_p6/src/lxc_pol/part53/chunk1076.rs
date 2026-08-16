//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1076/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1076<F: Float>(t7901: F, t8764: F, t1519: F, t32825: F, t33906: F, t33910: F, t33914: F, t33916: F, t33920: F, t33977: F, t34444: F, t34447: F, t34449: F, t34462: F, t569: F, t6985: F, t8158: F, t8463: F) -> F {
    let t34464 = t8764 * t7901;
    let t34466 = -F::cast_from(2.0_f64) * t1519 * t32825 + t34462 * t569 - F::cast_from(2.0_f64) * t6985 * t8158 - F::cast_from(2.0_f64) * t33906 + t33910 + t33914 - t33916 + t33920 + t33977 - F::cast_from(2.0_f64) * t34444 - F::cast_from(2.0_f64) * t34447 - F::cast_from(2.0_f64) * t34449 + F::cast_from(3.0_f64) * t34464 - t8463;
    t34466
}
