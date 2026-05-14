//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1106/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1106<F: Float>(t93449: F, t1322: F, t2999: F, t89: F, t23077: F, t375: F, t1636: F, t5700: F, t1586: F, t22862: F, t1637: F, t5696: F, t23043: F, t376: F, t23039: F, t1316: F, t1771: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t93450 = 2.0 / 27.0 * t93449;
    let t93452 = t89 * t2999 * t1322;
    let t93453 = 28.0 / 81.0 * t93452;
    let t93455 = t89 * t375 * t23077;
    let t93456 = t93455 / 3.0;
    let t93458 = t89 * t1636 * t5700;
    let t93468 = t1586 * t22862;
    let t93474 = t89 * t1637 * t5696;
    let t93477 = t89 * t376 * t23043;
    let t93478 = 2.0 / 3.0 * t93477;
    let t93480 = t89 * t376 * t23039;
    let t93481 = 4.0 / 3.0 * t93480;
    let t93503 = t1316 * t1771;
    (t93450, t93452, t93453, t93455, t93456, t93458, t93468, t93474, t93477, t93478, t93480, t93481, t93503)
}
