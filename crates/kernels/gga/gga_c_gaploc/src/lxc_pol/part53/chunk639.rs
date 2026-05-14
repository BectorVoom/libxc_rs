//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 639/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk639<F: Float>(t13779: F, t587: F, t13778: F, t2488: F, t2487: F, t12079: F, t901: F, t2366: F, t3689: F, t2365: F, t1429: F, t12533: F, t12536: F, t12065: F, t895: F, t11986: F, t874: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13780 = t587 * t13779;
    let t13782 = t2488 * t13778;
    let t13783 = t2487 * t13782;
    let t13789 = t12079 * t901;
    let t13791 = t2366 * t3689;
    let t13792 = t2365 * t13791;
    let t13793 = t1429 * t13792;
    let t13795 = 0.38342925953920749677e0 * t12533;
    let t13796 = 0.38342925953920749677e0 * t12536;
    let t13798 = t895 * t12065;
    let t13800 = t11986 * t874;
    (t13780, t13782, t13783, t13789, t13791, t13792, t13793, t13795, t13796, t13798, t13800)
}
