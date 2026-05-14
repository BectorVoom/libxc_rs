//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1064/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1064<F: Float>(t190: F, t2371: F, t251: F, t36452: F, t37991: F, t2344: F, t665: F, t7514: F, t675: F, t9567: F, t626: F, t703: F, t1526: F, t2322: F, t342: F, t657: F, t8639: F) -> (F, F, F, F, F, F, F) {
    let t42050 = 1.0 / t251 / t37991 / t190 / t2371 / t36452 / 96.0;
    let t42109 = t2344 * t2371;
    let t42123 = t665 * t7514;
    let t42163 = t9567 * t675;
    let t42262 = t626 * t703;
    let t42264 = t1526 * t42262 * t2322;
    let t42293 = 5.0 / 54.0 * t342 * t8639 * t657;
    (t42050, t42109, t42123, t42163, t42262, t42264, t42293)
}
