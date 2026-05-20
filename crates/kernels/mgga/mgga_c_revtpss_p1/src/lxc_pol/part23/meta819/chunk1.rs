//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2668/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2668<F: Float>(t1065: F, t19380: F, t1062: F, t19463: F, t11710: F, t19730: F, t3091: F, t20050: F, t3188: F, t20054: F, t1063: F, t18946: F, t247: F, t3109: F) -> (F, F, F, F, F, F) {
    let t65712 = t1065 * t19380;
    let t65717 = t19463 * t1062;
    let t65738 = t3091 * t11710 * t19730;
    let t65801 = t3188 * t20050;
    let t65803 = t3188 * t20054;
    let t65807 = t1063 * t247 * t3109 * t18946;
    (t65712, t65717, t65738, t65801, t65803, t65807)
}
