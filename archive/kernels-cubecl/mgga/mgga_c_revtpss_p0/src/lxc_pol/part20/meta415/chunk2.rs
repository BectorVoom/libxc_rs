//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1537/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1537<F: Float>(t1054: F, t11970: F, t11986: F, t828: F, t3091: F, t3096: F, t12097: F, t3090: F, t11631: F, t905: F, t606: F, t11280: F, t3127: F, t3172: F) -> (F, F, F, F, F) {
    let t43238 = t1054 * t11970;
    let t43240 = t828 * t11986;
    let t43242 = t3091 * t43240 * t3096;
    let t43244 = t12097 * t3090;
    let t43253 = t11631 * t905;
    let t43254 = t43253 * t606;
    let t43266 = t3127 * t3172 * t11280;
    (t43238, t43242, t43244, t43254, t43266)
}
