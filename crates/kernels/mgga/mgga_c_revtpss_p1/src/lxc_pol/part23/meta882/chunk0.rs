//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2792/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2792<F: Float>(t10090: F, t122: F, t14144: F, t2482: F, t6861: F, t72: F, t9994: F, t14145: F, t4114: F, t10014: F, t22336: F, t1398: F, t73820: F) -> (F, F, F, F) {
    let t75035 = t2482 * t10090 * t6861 * t9994 * t72 * t122 * t14144;
    let t75039 = t2482 * t4114 * t6861 * t14145;
    let t75041 = t10014 * t22336;
    let t75047 = t73820 * t1398;
    (t75035, t75039, t75041, t75047)
}
