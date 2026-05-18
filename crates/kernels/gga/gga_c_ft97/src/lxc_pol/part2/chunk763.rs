//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 763/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk763<F: Float>(t1580: F, t920: F, t3194: F, t3193: F, t100: F, t8275: F, t103: F, t7763: F, t11437: F, t1651: F, t3199: F, t1902: F) -> (F, F, F, F) {
    let t11982 = t920 * t1580;
    let t11983 = t3194 * t11982;
    let t11984 = t3193 * t11983;
    let t11987 = t8275 * t100;
    let t11988 = t103 * t7763;
    let t11989 = t11988 * t11437;
    let t11990 = t11987 * t11989;
    let t11993 = t3199 * t1651;
    let t11994 = t1902 * t11993;
    (t11982, t11984, t11990, t11994)
}
