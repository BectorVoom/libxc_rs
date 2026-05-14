//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1188/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1188<F: Float>(t22572: F, t25709: F, t25708: F, t11119: F, t92642: F, t93014: F, t45144: F, t6426: F, t22619: F, t25779: F, t415: F, t100688: F, t100881: F, t100981: F, t101016: F, t101234: F, t1300: F, t1701: F, t22591: F, t22597: F, t22603: F, t22743: F, t22777: F, t25653: F, t37551: F, t428: F, t5538: F, t5540: F, t7889: F, t93271: F, t93319: F, t93322: F, t93326: F, t938: F) -> (F, F, F, F, F, F, F) {
    let t101504 = t22572 * t25709;
    let t101505 = t25708 * t101504;
    let t101507 = t11119 * t92642;
    let t101512 = t11119 * t93014;
    let t101523 = t6426 * t45144;
    let t101532 = 0.29693535778629056444e-3 * t22619 * t415 * t25779;
    let t101567 = -0.88910709717637694816e-2 * t7889 * t22591 * t101016 * t428 + 0.51690243689028715488e-4 * t37551 * t5540 * t101523 + 0.27568129967481981594e-3 * t22603 * t22777 * t25653 + 0.10338048737805743098e-3 * t22597 * t5540 * t101234 - 0.51690243689028715488e-4 * t22603 * t5540 * t100981 - 0.10338048737805743098e-4 * t5538 * t5540 * t100688 - 0.3443640424494650102e-5 * t5538 * t22743 * t100881 - 0.85124811172839506173e-2 * t93319 + 0.85124811172839506173e-2 * t93322 - 0.10091343167942740398e-3 * t93326 + 0.75080154872671831175e-1 * t1300 * t1701 * t93271 * t938;
    (t101504, t101505, t101507, t101512, t101523, t101532, t101567)
}
