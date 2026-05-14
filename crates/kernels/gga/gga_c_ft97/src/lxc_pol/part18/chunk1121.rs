//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1121/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1121<F: Float>(t5824: F, t92968: F, t22632: F, t23774: F, t23775: F, t5829: F, t5830: F, t92557: F, t22642: F, t23834: F, t23839: F, t128: F, t1691: F, t23722: F, t14: F, t2057: F) -> (F, F, F, F, F, F, F) {
    let t94716 = t5824 * t92968;
    let t94719 = t23774 * t22632 * t23775;
    let t94722 = t5829 * t92557 * t5830;
    let t94753 = t22642 * t23834;
    let t94754 = t23839 * t94753;
    let t94760 = t128 * t1691;
    let t94761 = t94760 * t23722;
    let t94765 = t2057 * t14;
    (t94716, t94719, t94722, t94753, t94754, t94761, t94765)
}
