//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1016/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1016<F: Float>(t10688: F, t10712: F, t2749: F, t2770: F, t2811: F, t8232: F, t10516: F, t10666: F, t10704: F, t10799: F, t15229: F, t1901: F, t2739: F, t2862: F, t2867: F, t2894: F, t296: F, t319: F, t43409: F, t43513: F, t44352: F, t44360: F, t446: F, t824: F, t840: F, t871: F) -> (F, F) {
    let t44362 = t10688 * t10712;
    let t44369 = t2770 * t2749;
    let t44381 = t8232 * t2811;
    let t44387 = 4.0 / 3.0 * t446 * t840 * t871 * t10799 * t824 - 8.0 * t446 * t296 * t44352 - 8.0 * t446 * t2862 * t2749 * t10516 + 4.0 / 9.0 * t44360 + 8.0 * t446 * t296 * t44362 - 8.0 / 3.0 * t1901 * t15229 * t43409 - 8.0 / 3.0 * t1901 * t44369 * t10704 - 2.0 * t446 * t840 * t2894 * t2739 + 2.0 * t446 * t2862 * t319 * t43513 - 8.0 / 9.0 * t44381 + 4.0 * t446 * t840 * t10666 * t2867;
    (t44362, t44387)
}
