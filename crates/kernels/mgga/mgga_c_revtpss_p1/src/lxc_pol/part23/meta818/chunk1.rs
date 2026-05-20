//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2666/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2666<F: Float>(t11274: F, t20029: F, t11656: F, t19920: F, t11262: F, t3127: F, t6262: F, t15817: F, t4820: F, t15775: F, t4834: F, t1032: F, t1040: F, t19856: F) -> (F, F, F, F, F, F) {
    let t65585 = t11274 * t20029;
    let t65589 = t11656 * t19920;
    let t65596 = t3127 * t11262 * t6262;
    let t65598 = t15817 * t4820;
    let t65610 = t4834 * t15775;
    let t65613 = t19856 * t1032 * t1040;
    (t65585, t65589, t65596, t65598, t65610, t65613)
}
