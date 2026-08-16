//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 710/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk710<F: Float>(t1909: F, t20430: F, t16076: F, t925: F, t16228: F, t8217: F, t11939: F, t16336: F, t16337: F, t16338: F, t16342: F, t16343: F, t16346: F, t20101: F, t20116: F, t20136: F, t20151: F, t20159: F, t20316: F) -> (F, F, F, F, F, F) {
    let t20431 = t1909 * t20430;
    let t20434 = t16076 * t925;
    let t20435 = t1909 * t20434;
    let t20438 = t16228 * t925;
    let t20439 = t8217 * t20438;
    let t20448 = t16336 - t16337 + t16338 - t20101 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) * t20116 + t16342 - t16343 - t16346 - t20316 / F::cast_from(4.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t20136 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t20151 - t20159 / F::cast_from(9.0_f64) - t11939;
    (t20431, t20434, t20435, t20438, t20439, t20448)
}
