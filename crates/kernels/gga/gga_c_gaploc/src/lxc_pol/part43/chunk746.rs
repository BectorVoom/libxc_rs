//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 746/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk746<F: Float>(t10532: F, t10533: F, t41965: F, t10557: F, t30936: F, t12987: F, t7014: F, t1645: F, t2859: F, t9152: F, t3149: F, t8063: F, t2877: F, t9487: F, t12900: F, t4950: F) -> (F, F, F, F, F, F, F) {
    let t42245 = 0.27606906686822939767e2 * t10532 * t10533 * t41965;
    let t42250 = 0.17875244975925213335e2 * t10557 * t30936;
    let t42256 = t7014 * t12987;
    let t42257 = 0.15976219147466979032e-1 * t42256;
    let t42263 = 0.10725146985555128001e1 * t2859 * t1645 * t9152;
    let t42265 = 0.23833659967900284446e0 * t3149 * t8063;
    let t42267 = 0.35750489951850426669e0 * t9487 * t2877;
    let t42272 = 0.71500979903700853338e0 * t4950 * t12900;
    (t42245, t42250, t42257, t42263, t42265, t42267, t42272)
}
