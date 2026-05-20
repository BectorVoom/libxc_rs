//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1480/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1480<F: Float>(t12587: F, t6748: F, t3857: F, t6801: F, t3860: F, t3863: F, t123: F, t2630: F, t6800: F, t2608: F, t512: F, t1317: F, t22195: F) -> (F, F, F, F, F, F, F) {
    let t73252 = t6748 * t12587;
    let t73321 = t3857 * t6801;
    let t73329 = t3860 * t6801;
    let t73331 = t3863 * t6801;
    let t73341 = t6800 * t123 * t2630;
    let t73350 = t512 * t6800 * t2608;
    let t73360 = t1317 * t22195;
    (t73252, t73321, t73329, t73331, t73341, t73350, t73360)
}
