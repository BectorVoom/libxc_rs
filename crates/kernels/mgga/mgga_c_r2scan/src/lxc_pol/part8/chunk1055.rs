//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1055/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1055<F: Float>(t10549: F, t374: F, t2266: F, t2854: F, t3016: F, t2892: F, t2858: F, t10265: F, t10395: F, t372: F, t4703: F, t4721: F, t4880: F, t4882: F, t4887: F, t4891: F, t4897: F, t4901: F, t4964: F, t4967: F, t4972: F, t4975: F, t9883: F, t9884: F, t9885: F) -> (F, F, F, F, F) {
    let t10550 = t10549 * t374;
    let t10552 = t2266 * t2854 * t3016;
    let t10553 = 9.0 * t10552;
    let t10554 = t2854 * t2892;
    let t10555 = t2858 * t10554;
    let t10556 = 18.0 * t10555;
    let t10559 = t372 * t10265 - t10395 - t4703 - t4721 - t4880 - t4882 + t4887 + t4891 - t4897 - t4901 + t4964 - t4967 - t4972 + t4975 - t9883 + t9884 + t9885;
    (t10550, t10553, t10554, t10556, t10559)
}
