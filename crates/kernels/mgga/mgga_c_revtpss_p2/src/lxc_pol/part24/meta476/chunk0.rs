//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1460/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1460<F: Float>(t11262: F, t3127: F, t6262: F, t3160: F, t65338: F, t1062: F, t19463: F, t15711: F, t4834: F, t1041: F, t6301: F, t3150: F, t6307: F) -> (F, F, F, F, F, F) {
    let t65596 = t3127 * t11262 * t6262;
    let t65654 = t65338 * t3160;
    let t65717 = t19463 * t1062;
    let t65859 = t4834 * t15711;
    let t66022 = t1041 * t11262 * t6301;
    let t66029 = t3150 * t11262 * t6307;
    (t65596, t65654, t65717, t65859, t66022, t66029)
}
