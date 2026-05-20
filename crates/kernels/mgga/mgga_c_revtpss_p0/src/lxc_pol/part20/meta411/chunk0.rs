//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1521/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1521<F: Float>(t3154: F, t42871: F, t1036: F, t11240: F, t42646: F, t11268: F, t3173: F, t1063: F, t11232: F, t3172: F, t11982: F, t11285: F, t3127: F) -> (F, F, F, F, F, F, F) {
    let t42872 = t3154 * t3154;
    let t42873 = t42871 * t42872;
    let t42879 = t11240 * t1036 * t42646;
    let t42883 = t11268 * t3173;
    let t42886 = t1063 * t3172 * t11232;
    let t42889 = t1063 * t3172 * t11982;
    let t42892 = t3127 * t3172 * t11285;
    (t42872, t42873, t42879, t42883, t42886, t42889, t42892)
}
