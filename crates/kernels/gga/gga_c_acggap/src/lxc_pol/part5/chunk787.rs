//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 787/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk787<F: Float>(t244: F, t2868: F, t2970: F, t2987: F, t883: F, t712: F, t902: F, t277: F, t229: F, t2643: F, t699: F, t715: F, t2958: F, t912: F, t762: F, t771: F, t777: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11679 = t2868 * t244;
    let t11681 = t2970 * t244;
    let t11683 = t883 * t2987;
    let t11696 = t712 * t902;
    let t11698 = t2970 * t277;
    let t11700 = t229 * t2643;
    let t11702 = t715 * t699;
    let t11704 = t2958 * t912;
    let t11708 = 36.0 * t777 * t762 * t771;
    (t11679, t11681, t11683, t11696, t11698, t11700, t11702, t11704, t11708)
}
