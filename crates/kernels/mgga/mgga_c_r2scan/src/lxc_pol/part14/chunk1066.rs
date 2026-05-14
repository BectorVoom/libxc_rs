//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1066/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1066<F: Float>(t11189: F, t3262: F, t40620: F, t12365: F, t41280: F, t41283: F, t41285: F, t41286: F, t41289: F, t41291: F, t41294: F, t41296: F, t41300: F, t41305: F, t41308: F, t41311: F, t41314: F, t41316: F, t41319: F, t885: F) -> (F, F) {
    let t41322 = 135.0 / 64.0 * t3262 * t11189 * t40620;
    let t41323 = 2.0 * t12365 * t885 + t41280 - t41283 + t41285 + t41286 - t41289 - t41291 + t41294 + t41296 - t41300 - t41305 - t41308 + t41311 - t41314 - t41316 + t41319 + t41322;
    (t41322, t41323)
}
