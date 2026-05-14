//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1181/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1181<F: Float>(t14267: F, t4954: F, t16951: F, t3788: F, t4958: F, t57327: F, t57330: F, t57332: F, t57335: F, t57337: F, t57343: F, t57346: F, t57349: F, t57351: F, t56955: F, t57229: F, t57521: F) -> (F, F, F, F) {
    let t57523 = 0.35089340384731224426e1 * t14267 * t4954;
    let t57525 = 0.23392893589820816284e1 * t3788 * t16951;
    let t57527 = 0.1038945353962551798e3 * t14267 * t4958;
    let t57528 = -t57523 - t57525 - t57327 - t57330 - t57332 - t57335 + t57337 - t57343 + t57346 + t57349 - t57351 - t57527;
    let t57530 = t56955 + t57229 + t57521 + t57528;
    (t57523, t57525, t57527, t57530)
}
