//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 655/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk655<F: Float>(t20420: F, t83: F, t110: F, t20098: F, t452: F, t4495: F, t986: F, t20203: F, t8424: F, t1909: F, t16076: F, t925: F, t16228: F, t8217: F, t11939: F, t16336: F, t16337: F, t16338: F, t16342: F, t16343: F, t16346: F, t20101: F, t20116: F, t20136: F, t20151: F, t20159: F, t20316: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20421 = t83 * t20420;
    let t20424 = t452 * t110 * t20098;
    let t20428 = t452 * t986 * t4495;
    let t20430 = t8424 * t20203;
    let t20431 = t1909 * t20430;
    let t20434 = t16076 * t925;
    let t20435 = t1909 * t20434;
    let t20438 = t16228 * t925;
    let t20439 = t8217 * t20438;
    let t20448 = t16336 - t16337 + t16338 - t20101 / 3.0 - 2.0 * t20116 + t16342 - t16343 - t16346 - t20316 / 4.0 + 4.0 / 9.0 * t20136 - 2.0 / 3.0 * t20151 - t20159 / 9.0 - t11939;
    (t20421, t20424, t20428, t20430, t20431, t20434, t20435, t20438, t20439, t20448)
}
