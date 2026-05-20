//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2751/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2751<F: Float>(t1204: F, t6695: F, t1276: F, t6573: F, t12587: F, t6748: F, t21635: F, t3801: F, t3857: F, t6801: F, t3860: F, t123: F, t2630: F, t6800: F) -> (F, F, F, F, F, F, F) {
    let t73222 = t1204 * t6695;
    let t73236 = t1276 * t6573;
    let t73252 = t6748 * t12587;
    let t73273 = t21635 * t3801;
    let t73321 = t3857 * t6801;
    let t73329 = t3860 * t6801;
    let t73341 = t6800 * t123 * t2630;
    (t73222, t73236, t73252, t73273, t73321, t73329, t73341)
}
